#![allow(clippy::format_collect)]

use crate::{
    Event, git_store::StatusEntry, task_inventory::TaskContexts, task_store::TaskSettingsLocation,
    *,
};
use buffer_diff::{
    BufferDiffEvent, CALCULATE_DIFF_TASK, DiffHunkSecondaryStatus, DiffHunkStatus,
    DiffHunkStatusKind, assert_hunks,
};
use fs::FakeFs;
use futures::{StreamExt, future};
use git::{
    GitHostingProviderRegistry,
    repository::RepoPath,
    status::{StatusCode, TrackedStatus},
};
use git2::RepositoryInitOptions;
use gpui::{App, BackgroundExecutor, SemanticVersion, UpdateGlobal};
use http_client::Url;
use language::{
    Diagnostic, DiagnosticEntry, DiagnosticSet, DiskState, FakeLspAdapter, LanguageConfig,
    LanguageMatcher, LanguageName, LineEnding, OffsetRangeExt, Point, ToPoint,
    language_settings::{AllLanguageSettings, LanguageSettingsContent, language_settings},
    tree_sitter_rust, tree_sitter_typescript,
};
use lsp::{
    DiagnosticSeverity, DocumentChanges, FileOperationFilter, NumberOrString, TextDocumentEdit,
    WillRenameFiles, notification::DidRenameFiles,
};
use parking_lot::Mutex;
use paths::{config_dir, tasks_file};
use postage::stream::Stream as _;
use pretty_assertions::{assert_eq, assert_matches};
use rand::{Rng as _, rngs::StdRng};
use serde_json::json;
#[cfg(not(windows))]
use std::os;
use std::{env, mem, num::NonZeroU32, ops::Range, str::FromStr, sync::OnceLock, task::Poll};
use task::{ResolvedTask, TaskContext};
use unindent::Unindent as _;
use util::{
    TryFutureExt as _, assert_set_eq, maybe, path,
    paths::PathMatcher,
    test::{TempTree, marked_text_offsets},
    uri,
};
use worktree::WorktreeModelHandle as _;

#[gpui::test]
async fn test_block_via_channel(cx: &mut gpui::TestAppContext) {
    cx.executor().allow_parking();

    let (tx, mut rx) = futures::channel::mpsc::unbounded();
    let _thread = std::thread::spawn(move || {
        #[cfg(not(target_os = "windows"))]
        std::fs::metadata("/tmp").unwrap();
        #[cfg(target_os = "windows")]
        std::fs::metadata("C:/Windows").unwrap();
        std::thread::sleep(Duration::from_millis(1000));
        tx.unbounded_send(1).unwrap();
    });
    rx.next().await.unwrap();
}

#[gpui::test]
async fn test_block_via_smol(cx: &mut gpui::TestAppContext) {
    cx.executor().allow_parking();

    let io_task = smol::unblock(move || {
        println!("sleeping on thread {:?}", std::thread::current().id());
        std::thread::sleep(Duration::from_millis(10));
        1
    });

    let task = cx.foreground_executor().spawn(async move {
        io_task.await;
    });

    task.await;
}

#[cfg(not(windows))]
#[gpui::test]
async fn test_symlinks(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let dir = TempTree::new(json!({
        "root": {
            "apple": "",
            "banana": {
                "carrot": {
                    "date": "",
                    "endive": "",
                }
            },
            "fennel": {
                "grape": "",
            }
        }
    }));

    let root_link_path = dir.path().join("root_link");
    os::unix::fs::symlink(dir.path().join("root"), &root_link_path).unwrap();
    os::unix::fs::symlink(
        dir.path().join("root/fennel"),
        dir.path().join("root/finnochio"),
    )
    .unwrap();

    let project = Project::test(
        Arc::new(RealFs::new(None, cx.executor())),
        [root_link_path.as_ref()],
        cx,
    )
    .await;

    project.update(cx, |project, cx| {
        let tree = project.worktrees(cx).next().unwrap().read(cx);
        assert_eq!(tree.file_count(), 5);
        assert_eq!(
            tree.inode_for_path("fennel/grape"),
            tree.inode_for_path("finnochio/grape")
        );
    });
}

#[gpui::test]
async fn test_editorconfig_support(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let dir = TempTree::new(json!({
        ".editorconfig": r#"
        root = true
        [*.rs]
            indent_style = tab
            indent_size = 3
            end_of_line = lf
            insert_final_newline = true
            trim_trailing_whitespace = true
        [*.js]
            tab_width = 10
        "#,
        ".zed": {
            "settings.json": r#"{
                "tab_size": 8,
                "hard_tabs": false,
                "ensure_final_newline_on_save": false,
                "remove_trailing_whitespace_on_save": false,
                "soft_wrap": "editor_width"
            }"#,
        },
        "a.rs": "fn a() {\n    A\n}",
        "b": {
            ".editorconfig": r#"
            [*.rs]
                indent_size = 2
            "#,
            "b.rs": "fn b() {\n    B\n}",
        },
        "c.js": "def c\n  C\nend",
        "README.json": "tabs are better\n",
    }));

    let path = dir.path();
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree_from_real_fs(path, path).await;
    let project = Project::test(fs, [path], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(js_lang());
    language_registry.add(json_lang());
    language_registry.add(rust_lang());

    let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());

    cx.executor().run_until_parked();

    cx.update(|cx| {
        let tree = worktree.read(cx);
        let settings_for = |path: &str| {
            let file_entry = tree.entry_for_path(path).unwrap().clone();
            let file = File::for_entry(file_entry, worktree.clone());
            let file_language = project
                .read(cx)
                .languages()
                .language_for_file_path(file.path.as_ref());
            let file_language = cx
                .background_executor()
                .block(file_language)
                .expect("Failed to get file language");
            let file = file as _;
            language_settings(Some(file_language.name()), Some(&file), cx).into_owned()
        };

        let settings_a = settings_for("a.rs");
        let settings_b = settings_for("b/b.rs");
        let settings_c = settings_for("c.js");
        let settings_readme = settings_for("README.json");

        // .editorconfig overrides .zed/settings
        assert_eq!(Some(settings_a.tab_size), NonZeroU32::new(3));
        assert_eq!(settings_a.hard_tabs, true);
        assert_eq!(settings_a.ensure_final_newline_on_save, true);
        assert_eq!(settings_a.remove_trailing_whitespace_on_save, true);

        // .editorconfig in b/ overrides .editorconfig in root
        assert_eq!(Some(settings_b.tab_size), NonZeroU32::new(2));

        // "indent_size" is not set, so "tab_width" is used
        assert_eq!(Some(settings_c.tab_size), NonZeroU32::new(10));

        // README.md should not be affected by .editorconfig's globe "*.rs"
        assert_eq!(Some(settings_readme.tab_size), NonZeroU32::new(8));
    });
}

#[gpui::test]
async fn test_git_provider_project_setting(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.update(|cx| {
        GitHostingProviderRegistry::default_global(cx);
        git_hosting_providers::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    let str_path = path!("/dir");
    let path = Path::new(str_path);

    fs.insert_tree(
        path!("/dir"),
        json!({
            ".zed": {
                "settings.json": r#"{
                    "git_hosting_providers": [
                        {
                            "provider": "gitlab",
                            "base_url": "https://google.com",
                            "name": "foo"
                        }
                    ]
                }"#
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let (_worktree, _) =
        project.read_with(cx, |project, cx| project.find_worktree(path, cx).unwrap());
    cx.executor().run_until_parked();

    cx.update(|cx| {
        let provider = GitHostingProviderRegistry::global(cx);
        assert!(
            provider
                .list_hosting_providers()
                .into_iter()
                .any(|provider| provider.name() == "foo")
        );
    });

    fs.atomic_write(
        Path::new(path!("/dir/.zed/settings.json")).to_owned(),
        "{}".into(),
    )
    .await
    .unwrap();

    cx.run_until_parked();

    cx.update(|cx| {
        let provider = GitHostingProviderRegistry::global(cx);
        assert!(
            !provider
                .list_hosting_providers()
                .into_iter()
                .any(|provider| provider.name() == "foo")
        );
    });
}

#[gpui::test]
async fn test_managing_project_specific_settings(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    TaskStore::init(None);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            ".zed": {
                "settings.json": r#"{ "tab_size": 8 }"#,
                "tasks.json": r#"[{
                    "label": "cargo check all",
                    "command": "cargo",
                    "args": ["check", "--all"]
                },]"#,
            },
            "a": {
                "a.rs": "fn a() {\n    A\n}"
            },
            "b": {
                ".zed": {
                    "settings.json": r#"{ "tab_size": 2 }"#,
                    "tasks.json": r#"[{
                        "label": "cargo check",
                        "command": "cargo",
                        "args": ["check"]
                    },]"#,
                },
                "b.rs": "fn b() {\n  B\n}"
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());

    cx.executor().run_until_parked();
    let worktree_id = cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        })
    });

    let mut task_contexts = TaskContexts::default();
    task_contexts.active_worktree_context = Some((worktree_id, TaskContext::default()));
    let task_contexts = Arc::new(task_contexts);

    let topmost_local_task_source_kind = TaskSourceKind::Worktree {
        id: worktree_id,
        directory_in_worktree: PathBuf::from(".zed"),
        id_base: "local worktree tasks from directory \".zed\"".into(),
    };

    let all_tasks = cx
        .update(|cx| {
            let tree = worktree.read(cx);

            let file_a = File::for_entry(
                tree.entry_for_path("a/a.rs").unwrap().clone(),
                worktree.clone(),
            ) as _;
            let settings_a = language_settings(None, Some(&file_a), cx);
            let file_b = File::for_entry(
                tree.entry_for_path("b/b.rs").unwrap().clone(),
                worktree.clone(),
            ) as _;
            let settings_b = language_settings(None, Some(&file_b), cx);

            assert_eq!(settings_a.tab_size.get(), 8);
            assert_eq!(settings_b.tab_size.get(), 2);

            get_all_tasks(&project, task_contexts.clone(), cx)
        })
        .await
        .into_iter()
        .map(|(source_kind, task)| {
            let resolved = task.resolved;
            (
                source_kind,
                task.resolved_label,
                resolved.args,
                resolved.env,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        all_tasks,
        vec![
            (
                TaskSourceKind::Worktree {
                    id: worktree_id,
                    directory_in_worktree: PathBuf::from(path!("b/.zed")),
                    id_base: if cfg!(windows) {
                        "local worktree tasks from directory \"b\\\\.zed\"".into()
                    } else {
                        "local worktree tasks from directory \"b/.zed\"".into()
                    },
                },
                "cargo check".to_string(),
                vec!["check".to_string()],
                HashMap::default(),
            ),
            (
                topmost_local_task_source_kind.clone(),
                "cargo check all".to_string(),
                vec!["check".to_string(), "--all".to_string()],
                HashMap::default(),
            ),
        ]
    );

    let (_, resolved_task) = cx
        .update(|cx| get_all_tasks(&project, task_contexts.clone(), cx))
        .await
        .into_iter()
        .find(|(source_kind, _)| source_kind == &topmost_local_task_source_kind)
        .expect("should have one global task");
    project.update(cx, |project, cx| {
        let task_inventory = project
            .task_store
            .read(cx)
            .task_inventory()
            .cloned()
            .unwrap();
        task_inventory.update(cx, |inventory, _| {
            inventory.task_scheduled(topmost_local_task_source_kind.clone(), resolved_task);
            inventory
                .update_file_based_tasks(
                    TaskSettingsLocation::Global(tasks_file()),
                    Some(
                        &json!([{
                            "label": "cargo check unstable",
                            "command": "cargo",
                            "args": [
                                "check",
                                "--all",
                                "--all-targets"
                            ],
                            "env": {
                                "RUSTFLAGS": "-Zunstable-options"
                            }
                        }])
                        .to_string(),
                    ),
                )
                .unwrap();
        });
    });
    cx.run_until_parked();

    let all_tasks = cx
        .update(|cx| get_all_tasks(&project, task_contexts.clone(), cx))
        .await
        .into_iter()
        .map(|(source_kind, task)| {
            let resolved = task.resolved;
            (
                source_kind,
                task.resolved_label,
                resolved.args,
                resolved.env,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        all_tasks,
        vec![
            (
                topmost_local_task_source_kind.clone(),
                "cargo check all".to_string(),
                vec!["check".to_string(), "--all".to_string()],
                HashMap::default(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_id,
                    directory_in_worktree: PathBuf::from(path!("b/.zed")),
                    id_base: if cfg!(windows) {
                        "local worktree tasks from directory \"b\\\\.zed\"".into()
                    } else {
                        "local worktree tasks from directory \"b/.zed\"".into()
                    },
                },
                "cargo check".to_string(),
                vec!["check".to_string()],
                HashMap::default(),
            ),
            (
                TaskSourceKind::AbsPath {
                    abs_path: paths::tasks_file().clone(),
                    id_base: "global tasks.json".into(),
                },
                "cargo check unstable".to_string(),
                vec![
                    "check".to_string(),
                    "--all".to_string(),
                    "--all-targets".to_string(),
                ],
                HashMap::from_iter(Some((
                    "RUSTFLAGS".to_string(),
                    "-Zunstable-options".to_string()
                ))),
            ),
        ]
    );
}

#[gpui::test]
async fn test_fallback_to_single_worktree_tasks(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    TaskStore::init(None);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            ".zed": {
                "tasks.json": r#"[{
                    "label": "test worktree root",
                    "command": "echo $ZED_WORKTREE_ROOT"
                }]"#,
            },
            "a": {
                "a.rs": "fn a() {\n    A\n}"
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let _worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());

    cx.executor().run_until_parked();
    let worktree_id = cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        })
    });

    let active_non_worktree_item_tasks = cx
        .update(|cx| {
            get_all_tasks(
                &project,
                Arc::new(TaskContexts {
                    active_item_context: Some((Some(worktree_id), None, TaskContext::default())),
                    active_worktree_context: None,
                    other_worktree_contexts: Vec::new(),
                    lsp_task_sources: HashMap::default(),
                    latest_selection: None,
                }),
                cx,
            )
        })
        .await;
    assert!(
        active_non_worktree_item_tasks.is_empty(),
        "A task can not be resolved with context with no ZED_WORKTREE_ROOT data"
    );

    let active_worktree_tasks = cx
        .update(|cx| {
            get_all_tasks(
                &project,
                Arc::new(TaskContexts {
                    active_item_context: Some((Some(worktree_id), None, TaskContext::default())),
                    active_worktree_context: Some((worktree_id, {
                        let mut worktree_context = TaskContext::default();
                        worktree_context
                            .task_variables
                            .insert(task::VariableName::WorktreeRoot, "/dir".to_string());
                        worktree_context
                    })),
                    other_worktree_contexts: Vec::new(),
                    lsp_task_sources: HashMap::default(),
                    latest_selection: None,
                }),
                cx,
            )
        })
        .await;
    assert_eq!(
        active_worktree_tasks
            .into_iter()
            .map(|(source_kind, task)| {
                let resolved = task.resolved;
                (source_kind, resolved.command.unwrap())
            })
            .collect::<Vec<_>>(),
        vec![(
            TaskSourceKind::Worktree {
                id: worktree_id,
                directory_in_worktree: PathBuf::from(path!(".zed")),
                id_base: if cfg!(windows) {
                    "local worktree tasks from directory \".zed\"".into()
                } else {
                    "local worktree tasks from directory \".zed\"".into()
                },
            },
            "echo /dir".to_string(),
        )]
    );
}

#[gpui::test]
async fn test_managing_language_servers(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "test.rs": "const A: i32 = 1;",
            "test2.rs": "",
            "Cargo.toml": "a = 1",
            "package.json": "{\"a\": 1}",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    let mut fake_rust_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-rust-language-server",
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), "::".to_string()]),
                    ..Default::default()
                }),
                text_document_sync: Some(lsp::TextDocumentSyncCapability::Options(
                    lsp::TextDocumentSyncOptions {
                        save: Some(lsp::TextDocumentSyncSaveOptions::Supported(true)),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    let mut fake_json_servers = language_registry.register_fake_lsp(
        "JSON",
        FakeLspAdapter {
            name: "the-json-language-server",
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![":".to_string()]),
                    ..Default::default()
                }),
                text_document_sync: Some(lsp::TextDocumentSyncCapability::Options(
                    lsp::TextDocumentSyncOptions {
                        save: Some(lsp::TextDocumentSyncSaveOptions::Supported(true)),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    // Open a buffer without an associated language server.
    let (toml_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/Cargo.toml"), cx)
        })
        .await
        .unwrap();

    // Open a buffer with an associated language server before the language for it has been loaded.
    let (rust_buffer, _handle2) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/test.rs"), cx)
        })
        .await
        .unwrap();
    rust_buffer.update(cx, |buffer, _| {
        assert_eq!(buffer.language().map(|l| l.name()), None);
    });

    // Now we add the languages to the project, and ensure they get assigned to all
    // the relevant open buffers.
    language_registry.add(json_lang());
    language_registry.add(rust_lang());
    cx.executor().run_until_parked();
    rust_buffer.update(cx, |buffer, _| {
        assert_eq!(buffer.language().map(|l| l.name()), Some("Rust".into()));
    });

    // A server is started up, and it is notified about Rust files.
    let mut fake_rust_server = fake_rust_servers.next().await.unwrap();
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path(path!("/dir/test.rs")).unwrap(),
            version: 0,
            text: "const A: i32 = 1;".to_string(),
            language_id: "rust".to_string(),
        }
    );

    // The buffer is configured based on the language server's capabilities.
    rust_buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .completion_triggers()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>(),
            &[".".to_string(), "::".to_string()]
        );
    });
    toml_buffer.update(cx, |buffer, _| {
        assert!(buffer.completion_triggers().is_empty());
    });

    // Edit a buffer. The changes are reported to the language server.
    rust_buffer.update(cx, |buffer, cx| buffer.edit([(16..16, "2")], None, cx));
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidChangeTextDocument>()
            .await
            .text_document,
        lsp::VersionedTextDocumentIdentifier::new(
            lsp::Url::from_file_path(path!("/dir/test.rs")).unwrap(),
            1
        )
    );

    // Open a third buffer with a different associated language server.
    let (json_buffer, _json_handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/package.json"), cx)
        })
        .await
        .unwrap();

    // A json language server is started up and is only notified about the json buffer.
    let mut fake_json_server = fake_json_servers.next().await.unwrap();
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path(path!("/dir/package.json")).unwrap(),
            version: 0,
            text: "{\"a\": 1}".to_string(),
            language_id: "json".to_string(),
        }
    );

    // This buffer is configured based on the second language server's
    // capabilities.
    json_buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .completion_triggers()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>(),
            &[":".to_string()]
        );
    });

    // When opening another buffer whose language server is already running,
    // it is also configured based on the existing language server's capabilities.
    let (rust_buffer2, _handle4) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/test2.rs"), cx)
        })
        .await
        .unwrap();
    rust_buffer2.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .completion_triggers()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>(),
            &[".".to_string(), "::".to_string()]
        );
    });

    // Changes are reported only to servers matching the buffer's language.
    toml_buffer.update(cx, |buffer, cx| buffer.edit([(5..5, "23")], None, cx));
    rust_buffer2.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "let x = 1;")], None, cx)
    });
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidChangeTextDocument>()
            .await
            .text_document,
        lsp::VersionedTextDocumentIdentifier::new(
            lsp::Url::from_file_path(path!("/dir/test2.rs")).unwrap(),
            1
        )
    );

    // Save notifications are reported to all servers.
    project
        .update(cx, |project, cx| project.save_buffer(toml_buffer, cx))
        .await
        .unwrap();
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidSaveTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentIdentifier::new(
            lsp::Url::from_file_path(path!("/dir/Cargo.toml")).unwrap()
        )
    );
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidSaveTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentIdentifier::new(
            lsp::Url::from_file_path(path!("/dir/Cargo.toml")).unwrap()
        )
    );

    // Renames are reported only to servers matching the buffer's language.
    fs.rename(
        Path::new(path!("/dir/test2.rs")),
        Path::new(path!("/dir/test3.rs")),
        Default::default(),
    )
    .await
    .unwrap();
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidCloseTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path(path!("/dir/test2.rs")).unwrap()),
    );
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path(path!("/dir/test3.rs")).unwrap(),
            version: 0,
            text: rust_buffer2.update(cx, |buffer, _| buffer.text()),
            language_id: "rust".to_string(),
        },
    );

    rust_buffer2.update(cx, |buffer, cx| {
        buffer.update_diagnostics(
            LanguageServerId(0),
            DiagnosticSet::from_sorted_entries(
                vec![DiagnosticEntry {
                    diagnostic: Default::default(),
                    range: Anchor::MIN..Anchor::MAX,
                }],
                &buffer.snapshot(),
            ),
            cx,
        );
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, usize>(0..buffer.len(), false)
                .count(),
            1
        );
    });

    // When the rename changes the extension of the file, the buffer gets closed on the old
    // language server and gets opened on the new one.
    fs.rename(
        Path::new(path!("/dir/test3.rs")),
        Path::new(path!("/dir/test3.json")),
        Default::default(),
    )
    .await
    .unwrap();
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidCloseTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path(path!("/dir/test3.rs")).unwrap()),
    );
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path(path!("/dir/test3.json")).unwrap(),
            version: 0,
            text: rust_buffer2.update(cx, |buffer, _| buffer.text()),
            language_id: "json".to_string(),
        },
    );

    // We clear the diagnostics, since the language has changed.
    rust_buffer2.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, usize>(0..buffer.len(), false)
                .count(),
            0
        );
    });

    // The renamed file's version resets after changing language server.
    rust_buffer2.update(cx, |buffer, cx| buffer.edit([(0..0, "// ")], None, cx));
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidChangeTextDocument>()
            .await
            .text_document,
        lsp::VersionedTextDocumentIdentifier::new(
            lsp::Url::from_file_path(path!("/dir/test3.json")).unwrap(),
            1
        )
    );

    // Restart language servers
    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers(
            vec![rust_buffer.clone(), json_buffer.clone()],
            HashSet::default(),
            cx,
        );
    });

    let mut rust_shutdown_requests = fake_rust_server
        .set_request_handler::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
    let mut json_shutdown_requests = fake_json_server
        .set_request_handler::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
    futures::join!(rust_shutdown_requests.next(), json_shutdown_requests.next());

    let mut fake_rust_server = fake_rust_servers.next().await.unwrap();
    let mut fake_json_server = fake_json_servers.next().await.unwrap();

    // Ensure rust document is reopened in new rust language server
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path(path!("/dir/test.rs")).unwrap(),
            version: 0,
            text: rust_buffer.update(cx, |buffer, _| buffer.text()),
            language_id: "rust".to_string(),
        }
    );

    // Ensure json documents are reopened in new json language server
    assert_set_eq!(
        [
            fake_json_server
                .receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await
                .text_document,
            fake_json_server
                .receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await
                .text_document,
        ],
        [
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path(path!("/dir/package.json")).unwrap(),
                version: 0,
                text: json_buffer.update(cx, |buffer, _| buffer.text()),
                language_id: "json".to_string(),
            },
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path(path!("/dir/test3.json")).unwrap(),
                version: 0,
                text: rust_buffer2.update(cx, |buffer, _| buffer.text()),
                language_id: "json".to_string(),
            }
        ]
    );

    // Close notifications are reported only to servers matching the buffer's language.
    cx.update(|_| drop(_json_handle));
    let close_message = lsp::DidCloseTextDocumentParams {
        text_document: lsp::TextDocumentIdentifier::new(
            lsp::Url::from_file_path(path!("/dir/package.json")).unwrap(),
        ),
    };
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidCloseTextDocument>()
            .await,
        close_message,
    );
}

#[gpui::test]
async fn test_reporting_fs_changes_to_language_servers(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/the-root"),
        json!({
            ".gitignore": "target\n",
            "Cargo.lock": "",
            "src": {
                "a.rs": "",
                "b.rs": "",
            },
            "target": {
                "x": {
                    "out": {
                        "x.rs": ""
                    }
                },
                "y": {
                    "out": {
                        "y.rs": "",
                    }
                },
                "z": {
                    "out": {
                        "z.rs": ""
                    }
                }
            }
        }),
    )
    .await;
    fs.insert_tree(
        path!("/the-registry"),
        json!({
            "dep1": {
                "src": {
                    "dep1.rs": "",
                }
            },
            "dep2": {
                "src": {
                    "dep2.rs": "",
                }
            },
        }),
    )
    .await;
    fs.insert_tree(
        path!("/the/stdlib"),
        json!({
            "LICENSE": "",
            "src": {
                "string.rs": "",
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/the-root").as_ref()], cx).await;
    let (language_registry, lsp_store) = project.read_with(cx, |project, _| {
        (project.languages().clone(), project.lsp_store())
    });
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-language-server",
            ..Default::default()
        },
    );

    cx.executor().run_until_parked();

    // Start the language server by opening a buffer with a compatible file extension.
    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/src/a.rs"), cx)
        })
        .await
        .unwrap();

    // Initially, we don't load ignored files because the language server has not explicitly asked us to watch them.
    project.update(cx, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap();
        assert_eq!(
            worktree
                .read(cx)
                .snapshot()
                .entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_ignored))
                .collect::<Vec<_>>(),
            &[
                (Path::new(""), false),
                (Path::new(".gitignore"), false),
                (Path::new("Cargo.lock"), false),
                (Path::new("src"), false),
                (Path::new("src/a.rs"), false),
                (Path::new("src/b.rs"), false),
                (Path::new("target"), true),
            ]
        );
    });

    let prev_read_dir_count = fs.read_dir_call_count();

    let fake_server = fake_servers.next().await.unwrap();
    let (server_id, server_name) = lsp_store.read_with(cx, |lsp_store, _| {
        let (id, status) = lsp_store.language_server_statuses().next().unwrap();
        (id, LanguageServerName::from(status.name.as_str()))
    });

    // Simulate jumping to a definition in a dependency outside of the worktree.
    let _out_of_worktree_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                lsp::Url::from_file_path(path!("/the-registry/dep1/src/dep1.rs")).unwrap(),
                server_id,
                server_name.clone(),
                cx,
            )
        })
        .await
        .unwrap();

    // Keep track of the FS events reported to the language server.
    let file_changes = Arc::new(Mutex::new(Vec::new()));
    fake_server
        .request::<lsp::request::RegisterCapability>(lsp::RegistrationParams {
            registrations: vec![lsp::Registration {
                id: Default::default(),
                method: "workspace/didChangeWatchedFiles".to_string(),
                register_options: serde_json::to_value(
                    lsp::DidChangeWatchedFilesRegistrationOptions {
                        watchers: vec![
                            lsp::FileSystemWatcher {
                                glob_pattern: lsp::GlobPattern::String(
                                    path!("/the-root/Cargo.toml").to_string(),
                                ),
                                kind: None,
                            },
                            lsp::FileSystemWatcher {
                                glob_pattern: lsp::GlobPattern::String(
                                    path!("/the-root/src/*.{rs,c}").to_string(),
                                ),
                                kind: None,
                            },
                            lsp::FileSystemWatcher {
                                glob_pattern: lsp::GlobPattern::String(
                                    path!("/the-root/target/y/**/*.rs").to_string(),
                                ),
                                kind: None,
                            },
                            lsp::FileSystemWatcher {
                                glob_pattern: lsp::GlobPattern::String(
                                    path!("/the/stdlib/src/**/*.rs").to_string(),
                                ),
                                kind: None,
                            },
                            lsp::FileSystemWatcher {
                                glob_pattern: lsp::GlobPattern::String(
                                    path!("**/Cargo.lock").to_string(),
                                ),
                                kind: None,
                            },
                        ],
                    },
                )
                .ok(),
            }],
        })
        .await
        .into_response()
        .unwrap();
    fake_server.handle_notification::<lsp::notification::DidChangeWatchedFiles, _>({
        let file_changes = file_changes.clone();
        move |params, _| {
            let mut file_changes = file_changes.lock();
            file_changes.extend(params.changes);
            file_changes.sort_by(|a, b| a.uri.cmp(&b.uri));
        }
    });

    cx.executor().run_until_parked();
    assert_eq!(mem::take(&mut *file_changes.lock()), &[]);
    assert_eq!(fs.read_dir_call_count() - prev_read_dir_count, 5);

    let mut new_watched_paths = fs.watched_paths();
    new_watched_paths.retain(|path| !path.starts_with(config_dir()));
    assert_eq!(
        &new_watched_paths,
        &[
            Path::new(path!("/the-root")),
            Path::new(path!("/the-registry/dep1/src/dep1.rs")),
            Path::new(path!("/the/stdlib/src"))
        ]
    );

    // Now the language server has asked us to watch an ignored directory path,
    // so we recursively load it.
    project.update(cx, |project, cx| {
        let worktree = project.visible_worktrees(cx).next().unwrap();
        assert_eq!(
            worktree
                .read(cx)
                .snapshot()
                .entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_ignored))
                .collect::<Vec<_>>(),
            &[
                (Path::new(""), false),
                (Path::new(".gitignore"), false),
                (Path::new("Cargo.lock"), false),
                (Path::new("src"), false),
                (Path::new("src/a.rs"), false),
                (Path::new("src/b.rs"), false),
                (Path::new("target"), true),
                (Path::new("target/x"), true),
                (Path::new("target/y"), true),
                (Path::new("target/y/out"), true),
                (Path::new("target/y/out/y.rs"), true),
                (Path::new("target/z"), true),
            ]
        );
    });

    // Perform some file system mutations, two of which match the watched patterns,
    // and one of which does not.
    fs.create_file(path!("/the-root/src/c.rs").as_ref(), Default::default())
        .await
        .unwrap();
    fs.create_file(path!("/the-root/src/d.txt").as_ref(), Default::default())
        .await
        .unwrap();
    fs.remove_file(path!("/the-root/src/b.rs").as_ref(), Default::default())
        .await
        .unwrap();
    fs.create_file(
        path!("/the-root/target/x/out/x2.rs").as_ref(),
        Default::default(),
    )
    .await
    .unwrap();
    fs.create_file(
        path!("/the-root/target/y/out/y2.rs").as_ref(),
        Default::default(),
    )
    .await
    .unwrap();
    fs.save(
        path!("/the-root/Cargo.lock").as_ref(),
        &"".into(),
        Default::default(),
    )
    .await
    .unwrap();
    fs.save(
        path!("/the-stdlib/LICENSE").as_ref(),
        &"".into(),
        Default::default(),
    )
    .await
    .unwrap();
    fs.save(
        path!("/the/stdlib/src/string.rs").as_ref(),
        &"".into(),
        Default::default(),
    )
    .await
    .unwrap();

    // The language server receives events for the FS mutations that match its watch patterns.
    cx.executor().run_until_parked();
    assert_eq!(
        &*file_changes.lock(),
        &[
            lsp::FileEvent {
                uri: lsp::Url::from_file_path(path!("/the-root/Cargo.lock")).unwrap(),
                typ: lsp::FileChangeType::CHANGED,
            },
            lsp::FileEvent {
                uri: lsp::Url::from_file_path(path!("/the-root/src/b.rs")).unwrap(),
                typ: lsp::FileChangeType::DELETED,
            },
            lsp::FileEvent {
                uri: lsp::Url::from_file_path(path!("/the-root/src/c.rs")).unwrap(),
                typ: lsp::FileChangeType::CREATED,
            },
            lsp::FileEvent {
                uri: lsp::Url::from_file_path(path!("/the-root/target/y/out/y2.rs")).unwrap(),
                typ: lsp::FileChangeType::CREATED,
            },
            lsp::FileEvent {
                uri: lsp::Url::from_file_path(path!("/the/stdlib/src/string.rs")).unwrap(),
                typ: lsp::FileChangeType::CHANGED,
            },
        ]
    );
}

#[gpui::test]
async fn test_single_file_worktrees_diagnostics(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.rs": "let a = 1;",
            "b.rs": "let b = 2;"
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [path!("/dir/a.rs").as_ref(), path!("/dir/b.rs").as_ref()],
        cx,
    )
    .await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());

    let buffer_a = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();
    let buffer_b = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/dir/b.rs"), cx)
        })
        .await
        .unwrap();

    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store
            .update_diagnostics(
                LanguageServerId(0),
                lsp::PublishDiagnosticsParams {
                    uri: Url::from_file_path(path!("/dir/a.rs")).unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 5)),
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "error 1".to_string(),
                        ..Default::default()
                    }],
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
        lsp_store
            .update_diagnostics(
                LanguageServerId(0),
                lsp::PublishDiagnosticsParams {
                    uri: Url::from_file_path(path!("/dir/b.rs")).unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 5)),
                        severity: Some(DiagnosticSeverity::WARNING),
                        message: "error 2".to_string(),
                        ..Default::default()
                    }],
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
    });

    buffer_a.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let ", None),
                ("a", Some(DiagnosticSeverity::ERROR)),
                (" = 1;", None),
            ]
        );
    });
    buffer_b.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let ", None),
                ("b", Some(DiagnosticSeverity::WARNING)),
                (" = 2;", None),
            ]
        );
    });
}

#[gpui::test]
async fn test_omitted_diagnostics(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "dir": {
                ".git": {
                    "HEAD": "ref: refs/heads/main",
                },
                ".gitignore": "b.rs",
                "a.rs": "let a = 1;",
                "b.rs": "let b = 2;",
            },
            "other.rs": "let b = c;"
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/root/dir").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/root/dir"), true, cx)
        })
        .await
        .unwrap();
    let main_worktree_id = worktree.read_with(cx, |tree, _| tree.id());

    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/root/other.rs"), false, cx)
        })
        .await
        .unwrap();
    let other_worktree_id = worktree.update(cx, |tree, _| tree.id());

    let server_id = LanguageServerId(0);
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store
            .update_diagnostics(
                server_id,
                lsp::PublishDiagnosticsParams {
                    uri: Url::from_file_path(path!("/root/dir/b.rs")).unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 5)),
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "unused variable 'b'".to_string(),
                        ..Default::default()
                    }],
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
        lsp_store
            .update_diagnostics(
                server_id,
                lsp::PublishDiagnosticsParams {
                    uri: Url::from_file_path(path!("/root/other.rs")).unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 8), lsp::Position::new(0, 9)),
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "unknown variable 'c'".to_string(),
                        ..Default::default()
                    }],
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
    });

    let main_ignored_buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((main_worktree_id, "b.rs"), cx)
        })
        .await
        .unwrap();
    main_ignored_buffer.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let ", None),
                ("b", Some(DiagnosticSeverity::ERROR)),
                (" = 2;", None),
            ],
            "Gigitnored buffers should still get in-buffer diagnostics",
        );
    });
    let other_buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((other_worktree_id, ""), cx)
        })
        .await
        .unwrap();
    other_buffer.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let b = ", None),
                ("c", Some(DiagnosticSeverity::ERROR)),
                (";", None),
            ],
            "Buffers from hidden projects should still get in-buffer diagnostics"
        );
    });

    project.update(cx, |project, cx| {
        assert_eq!(project.diagnostic_summaries(false, cx).next(), None);
        assert_eq!(
            project.diagnostic_summaries(true, cx).collect::<Vec<_>>(),
            vec![(
                ProjectPath {
                    worktree_id: main_worktree_id,
                    path: Arc::from(Path::new("b.rs")),
                },
                server_id,
                DiagnosticSummary {
                    error_count: 1,
                    warning_count: 0,
                }
            )]
        );
        assert_eq!(project.diagnostic_summary(false, cx).error_count, 0);
        assert_eq!(project.diagnostic_summary(true, cx).error_count, 1);
    });
}

#[gpui::test]
async fn test_disk_based_diagnostics_progress(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let progress_token = "the-progress-token";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.rs": "fn a() { A }",
            "b.rs": "const y: i32 = 1",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            disk_based_diagnostics_progress_token: Some(progress_token.into()),
            disk_based_diagnostics_sources: vec!["disk".into()],
            ..Default::default()
        },
    );

    let worktree_id = project.update(cx, |p, cx| p.worktrees(cx).next().unwrap().read(cx).id());

    // Cause worktree to start the fake language server
    let _ = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/b.rs"), cx)
        })
        .await
        .unwrap();

    let mut events = cx.events(&project);

    let fake_server = fake_servers.next().await.unwrap();
    assert_eq!(
        events.next().await.unwrap(),
        Event::LanguageServerAdded(
            LanguageServerId(0),
            fake_server.server.name(),
            Some(worktree_id)
        ),
    );

    fake_server
        .start_progress(format!("{}/0", progress_token))
        .await;
    assert_eq!(events.next().await.unwrap(), Event::RefreshInlayHints);
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsStarted {
            language_server_id: LanguageServerId(0),
        }
    );

    fake_server.notify::<lsp::notification::PublishDiagnostics>(&lsp::PublishDiagnosticsParams {
        uri: Url::from_file_path(path!("/dir/a.rs")).unwrap(),
        version: None,
        diagnostics: vec![lsp::Diagnostic {
            range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            message: "undefined variable 'A'".to_string(),
            ..Default::default()
        }],
    });
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiagnosticsUpdated {
            language_server_id: LanguageServerId(0),
            path: (worktree_id, Path::new("a.rs")).into()
        }
    );

    fake_server.end_progress(format!("{}/0", progress_token));
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsFinished {
            language_server_id: LanguageServerId(0)
        }
    );

    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/a.rs"), cx))
        .await
        .unwrap();

    buffer.update(cx, |buffer, _| {
        let snapshot = buffer.snapshot();
        let diagnostics = snapshot
            .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
            .collect::<Vec<_>>();
        assert_eq!(
            diagnostics,
            &[DiagnosticEntry {
                range: Point::new(0, 9)..Point::new(0, 10),
                diagnostic: Diagnostic {
                    severity: lsp::DiagnosticSeverity::ERROR,
                    message: "undefined variable 'A'".to_string(),
                    group_id: 0,
                    is_primary: true,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            }]
        )
    });

    // Ensure publishing empty diagnostics twice only results in one update event.
    fake_server.notify::<lsp::notification::PublishDiagnostics>(&lsp::PublishDiagnosticsParams {
        uri: Url::from_file_path(path!("/dir/a.rs")).unwrap(),
        version: None,
        diagnostics: Default::default(),
    });
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiagnosticsUpdated {
            language_server_id: LanguageServerId(0),
            path: (worktree_id, Path::new("a.rs")).into()
        }
    );

    fake_server.notify::<lsp::notification::PublishDiagnostics>(&lsp::PublishDiagnosticsParams {
        uri: Url::from_file_path(path!("/dir/a.rs")).unwrap(),
        version: None,
        diagnostics: Default::default(),
    });
    cx.executor().run_until_parked();
    assert_eq!(futures::poll!(events.next()), Poll::Pending);
}

#[gpui::test]
async fn test_restarting_server_with_diagnostics_running(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let progress_token = "the-progress-token";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), json!({ "a.rs": "" })).await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-language-server",
            disk_based_diagnostics_sources: vec!["disk".into()],
            disk_based_diagnostics_progress_token: Some(progress_token.into()),
            ..Default::default()
        },
    );

    let worktree_id = project.update(cx, |p, cx| p.worktrees(cx).next().unwrap().read(cx).id());

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();
    // Simulate diagnostics starting to update.
    let fake_server = fake_servers.next().await.unwrap();
    fake_server.start_progress(progress_token).await;

    // Restart the server before the diagnostics finish updating.
    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers(vec![buffer], HashSet::default(), cx);
    });
    let mut events = cx.events(&project);

    // Simulate the newly started server sending more diagnostics.
    let fake_server = fake_servers.next().await.unwrap();
    assert_eq!(
        events.next().await.unwrap(),
        Event::LanguageServerRemoved(LanguageServerId(0))
    );
    assert_eq!(
        events.next().await.unwrap(),
        Event::LanguageServerAdded(
            LanguageServerId(1),
            fake_server.server.name(),
            Some(worktree_id)
        )
    );
    assert_eq!(events.next().await.unwrap(), Event::RefreshInlayHints);
    fake_server.start_progress(progress_token).await;
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsStarted {
            language_server_id: LanguageServerId(1)
        }
    );
    project.update(cx, |project, cx| {
        assert_eq!(
            project
                .language_servers_running_disk_based_diagnostics(cx)
                .collect::<Vec<_>>(),
            [LanguageServerId(1)]
        );
    });

    // All diagnostics are considered done, despite the old server's diagnostic
    // task never completing.
    fake_server.end_progress(progress_token);
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsFinished {
            language_server_id: LanguageServerId(1)
        }
    );
    project.update(cx, |project, cx| {
        assert_eq!(
            project
                .language_servers_running_disk_based_diagnostics(cx)
                .collect::<Vec<_>>(),
            [] as [language::LanguageServerId; 0]
        );
    });
}

#[gpui::test]
async fn test_restarting_server_with_diagnostics_published(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), json!({ "a.rs": "x" })).await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    let (buffer, _) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();

    // Publish diagnostics
    let fake_server = fake_servers.next().await.unwrap();
    fake_server.notify::<lsp::notification::PublishDiagnostics>(&lsp::PublishDiagnosticsParams {
        uri: Url::from_file_path(path!("/dir/a.rs")).unwrap(),
        version: None,
        diagnostics: vec![lsp::Diagnostic {
            range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            message: "the message".to_string(),
            ..Default::default()
        }],
    });

    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, usize>(0..1, false)
                .map(|entry| entry.diagnostic.message.clone())
                .collect::<Vec<_>>(),
            ["the message".to_string()]
        );
    });
    project.update(cx, |project, cx| {
        assert_eq!(
            project.diagnostic_summary(false, cx),
            DiagnosticSummary {
                error_count: 1,
                warning_count: 0,
            }
        );
    });

    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers(vec![buffer.clone()], HashSet::default(), cx);
    });

    // The diagnostics are cleared.
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, usize>(0..1, false)
                .map(|entry| entry.diagnostic.message.clone())
                .collect::<Vec<_>>(),
            Vec::<String>::new(),
        );
    });
    project.update(cx, |project, cx| {
        assert_eq!(
            project.diagnostic_summary(false, cx),
            DiagnosticSummary {
                error_count: 0,
                warning_count: 0,
            }
        );
    });
}

#[gpui::test]
async fn test_restarted_server_reporting_invalid_buffer_version(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), json!({ "a.rs": "" })).await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();

    // Before restarting the server, report diagnostics with an unknown buffer version.
    let fake_server = fake_servers.next().await.unwrap();
    fake_server.notify::<lsp::notification::PublishDiagnostics>(&lsp::PublishDiagnosticsParams {
        uri: lsp::Url::from_file_path(path!("/dir/a.rs")).unwrap(),
        version: Some(10000),
        diagnostics: Vec::new(),
    });
    cx.executor().run_until_parked();
    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers(vec![buffer.clone()], HashSet::default(), cx);
    });

    let mut fake_server = fake_servers.next().await.unwrap();
    let notification = fake_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await
        .text_document;
    assert_eq!(notification.version, 0);
}

#[gpui::test]
async fn test_cancel_language_server_work(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let progress_token = "the-progress-token";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), json!({ "a.rs": "" })).await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-language-server",
            disk_based_diagnostics_sources: vec!["disk".into()],
            disk_based_diagnostics_progress_token: Some(progress_token.into()),
            ..Default::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();

    // Simulate diagnostics starting to update.
    let mut fake_server = fake_servers.next().await.unwrap();
    fake_server
        .start_progress_with(
            "another-token",
            lsp::WorkDoneProgressBegin {
                cancellable: Some(false),
                ..Default::default()
            },
        )
        .await;
    fake_server
        .start_progress_with(
            progress_token,
            lsp::WorkDoneProgressBegin {
                cancellable: Some(true),
                ..Default::default()
            },
        )
        .await;
    cx.executor().run_until_parked();

    project.update(cx, |project, cx| {
        project.cancel_language_server_work_for_buffers([buffer.clone()], cx)
    });

    let cancel_notification = fake_server
        .receive_notification::<lsp::notification::WorkDoneProgressCancel>()
        .await;
    assert_eq!(
        cancel_notification.token,
        NumberOrString::String(progress_token.into())
    );
}

#[gpui::test]
async fn test_toggling_enable_language_server(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), json!({ "a.rs": "", "b.js": "" }))
        .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    let mut fake_rust_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "rust-lsp",
            ..Default::default()
        },
    );
    let mut fake_js_servers = language_registry.register_fake_lsp(
        "JavaScript",
        FakeLspAdapter {
            name: "js-lsp",
            ..Default::default()
        },
    );
    language_registry.add(rust_lang());
    language_registry.add(js_lang());

    let _rs_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();
    let _js_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/b.js"), cx)
        })
        .await
        .unwrap();

    let mut fake_rust_server_1 = fake_rust_servers.next().await.unwrap();
    assert_eq!(
        fake_rust_server_1
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document
            .uri
            .as_str(),
        uri!("file:///dir/a.rs")
    );

    let mut fake_js_server = fake_js_servers.next().await.unwrap();
    assert_eq!(
        fake_js_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document
            .uri
            .as_str(),
        uri!("file:///dir/b.js")
    );

    // Disable Rust language server, ensuring only that server gets stopped.
    cx.update(|cx| {
        SettingsStore::update_global(cx, |settings, cx| {
            settings.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                settings.languages.0.insert(
                    "Rust".into(),
                    LanguageSettingsContent {
                        enable_language_server: Some(false),
                        ..Default::default()
                    },
                );
            });
        })
    });
    fake_rust_server_1
        .receive_notification::<lsp::notification::Exit>()
        .await;

    // Enable Rust and disable JavaScript language servers, ensuring that the
    // former gets started again and that the latter stops.
    cx.update(|cx| {
        SettingsStore::update_global(cx, |settings, cx| {
            settings.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                settings.languages.0.insert(
                    LanguageName::new("Rust"),
                    LanguageSettingsContent {
                        enable_language_server: Some(true),
                        ..Default::default()
                    },
                );
                settings.languages.0.insert(
                    LanguageName::new("JavaScript"),
                    LanguageSettingsContent {
                        enable_language_server: Some(false),
                        ..Default::default()
                    },
                );
            });
        })
    });
    let mut fake_rust_server_2 = fake_rust_servers.next().await.unwrap();
    assert_eq!(
        fake_rust_server_2
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document
            .uri
            .as_str(),
        uri!("file:///dir/a.rs")
    );
    fake_js_server
        .receive_notification::<lsp::notification::Exit>()
        .await;
}

#[gpui::test(iterations = 3)]
async fn test_transforming_diagnostics(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let text = "
        fn a() { A }
        fn b() { BB }
        fn c() { CCC }
    "
    .unindent();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), json!({ "a.rs": text })).await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            disk_based_diagnostics_sources: vec!["disk".into()],
            ..Default::default()
        },
    );

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();

    let _handle = project.update(cx, |project, cx| {
        project.register_buffer_with_language_servers(&buffer, cx)
    });

    let mut fake_server = fake_servers.next().await.unwrap();
    let open_notification = fake_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;

    // Edit the buffer, moving the content down
    buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "\n\n")], None, cx));
    let change_notification_1 = fake_server
        .receive_notification::<lsp::notification::DidChangeTextDocument>()
        .await;
    assert!(change_notification_1.text_document.version > open_notification.text_document.version);

    // Report some diagnostics for the initial version of the buffer
    fake_server.notify::<lsp::notification::PublishDiagnostics>(&lsp::PublishDiagnosticsParams {
        uri: lsp::Url::from_file_path(path!("/dir/a.rs")).unwrap(),
        version: Some(open_notification.text_document.version),
        diagnostics: vec![
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "undefined variable 'A'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 9), lsp::Position::new(1, 11)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "undefined variable 'BB'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(2, 9), lsp::Position::new(2, 12)),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("disk".to_string()),
                message: "undefined variable 'CCC'".to_string(),
                ..Default::default()
            },
        ],
    });

    // The diagnostics have moved down since they were created.
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(Point::new(3, 0)..Point::new(5, 0), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(3, 9)..Point::new(3, 11),
                    diagnostic: Diagnostic {
                        source: Some("disk".into()),
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'BB'".to_string(),
                        is_disk_based: true,
                        group_id: 1,
                        is_primary: true,
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    },
                },
                DiagnosticEntry {
                    range: Point::new(4, 9)..Point::new(4, 12),
                    diagnostic: Diagnostic {
                        source: Some("disk".into()),
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'CCC'".to_string(),
                        is_disk_based: true,
                        group_id: 2,
                        is_primary: true,
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    }
                }
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, 0..buffer.len()),
            [
                ("\n\nfn a() { ".to_string(), None),
                ("A".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\nfn b() { ".to_string(), None),
                ("BB".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\nfn c() { ".to_string(), None),
                ("CCC".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\n".to_string(), None),
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, Point::new(3, 10)..Point::new(4, 11)),
            [
                ("B".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\nfn c() { ".to_string(), None),
                ("CC".to_string(), Some(DiagnosticSeverity::ERROR)),
            ]
        );
    });

    // Ensure overlapping diagnostics are highlighted correctly.
    fake_server.notify::<lsp::notification::PublishDiagnostics>(&lsp::PublishDiagnosticsParams {
        uri: lsp::Url::from_file_path(path!("/dir/a.rs")).unwrap(),
        version: Some(open_notification.text_document.version),
        diagnostics: vec![
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "undefined variable 'A'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 12)),
                severity: Some(DiagnosticSeverity::WARNING),
                message: "unreachable statement".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
        ],
    });

    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(Point::new(2, 0)..Point::new(3, 0), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(2, 9)..Point::new(2, 12),
                    diagnostic: Diagnostic {
                        source: Some("disk".into()),
                        severity: DiagnosticSeverity::WARNING,
                        message: "unreachable statement".to_string(),
                        is_disk_based: true,
                        group_id: 4,
                        is_primary: true,
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(2, 9)..Point::new(2, 10),
                    diagnostic: Diagnostic {
                        source: Some("disk".into()),
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'A'".to_string(),
                        is_disk_based: true,
                        group_id: 3,
                        is_primary: true,
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    },
                }
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, Point::new(2, 0)..Point::new(3, 0)),
            [
                ("fn a() { ".to_string(), None),
                ("A".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }".to_string(), Some(DiagnosticSeverity::WARNING)),
                ("\n".to_string(), None),
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, Point::new(2, 10)..Point::new(3, 0)),
            [
                (" }".to_string(), Some(DiagnosticSeverity::WARNING)),
                ("\n".to_string(), None),
            ]
        );
    });

    // Keep editing the buffer and ensure disk-based diagnostics get translated according to the
    // changes since the last save.
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "    ")], None, cx);
        buffer.edit(
            [(Point::new(2, 8)..Point::new(2, 10), "(x: usize)")],
            None,
            cx,
        );
        buffer.edit([(Point::new(3, 10)..Point::new(3, 10), "xxx")], None, cx);
    });
    let change_notification_2 = fake_server
        .receive_notification::<lsp::notification::DidChangeTextDocument>()
        .await;
    assert!(
        change_notification_2.text_document.version > change_notification_1.text_document.version
    );

    // Handle out-of-order diagnostics
    fake_server.notify::<lsp::notification::PublishDiagnostics>(&lsp::PublishDiagnosticsParams {
        uri: lsp::Url::from_file_path(path!("/dir/a.rs")).unwrap(),
        version: Some(change_notification_2.text_document.version),
        diagnostics: vec![
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 9), lsp::Position::new(1, 11)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "undefined variable 'BB'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                severity: Some(DiagnosticSeverity::WARNING),
                message: "undefined variable 'A'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
        ],
    });

    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(2, 21)..Point::new(2, 22),
                    diagnostic: Diagnostic {
                        source: Some("disk".into()),
                        severity: DiagnosticSeverity::WARNING,
                        message: "undefined variable 'A'".to_string(),
                        is_disk_based: true,
                        group_id: 6,
                        is_primary: true,
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(3, 9)..Point::new(3, 14),
                    diagnostic: Diagnostic {
                        source: Some("disk".into()),
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'BB'".to_string(),
                        is_disk_based: true,
                        group_id: 5,
                        is_primary: true,
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    },
                }
            ]
        );
    });
}

#[gpui::test]
async fn test_empty_diagnostic_ranges(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let text = concat!(
        "let one = ;\n", //
        "let two = \n",
        "let three = 3;\n",
    );

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({ "a.rs": text })).await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    project.update(cx, |project, cx| {
        project.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store
                .update_diagnostic_entries(
                    LanguageServerId(0),
                    PathBuf::from("/dir/a.rs"),
                    None,
                    None,
                    vec![
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(0, 10))
                                ..Unclipped(PointUtf16::new(0, 10)),
                            diagnostic: Diagnostic {
                                severity: DiagnosticSeverity::ERROR,
                                message: "syntax error 1".to_string(),
                                source_kind: DiagnosticSourceKind::Pushed,
                                ..Diagnostic::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(1, 10))
                                ..Unclipped(PointUtf16::new(1, 10)),
                            diagnostic: Diagnostic {
                                severity: DiagnosticSeverity::ERROR,
                                message: "syntax error 2".to_string(),
                                source_kind: DiagnosticSourceKind::Pushed,
                                ..Diagnostic::default()
                            },
                        },
                    ],
                    cx,
                )
                .unwrap();
        })
    });

    // An empty range is extended forward to include the following character.
    // At the end of a line, an empty range is extended backward to include
    // the preceding character.
    buffer.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let one = ", None),
                (";", Some(DiagnosticSeverity::ERROR)),
                ("\nlet two =", None),
                (" ", Some(DiagnosticSeverity::ERROR)),
                ("\nlet three = 3;\n", None)
            ]
        );
    });
}

#[gpui::test]
async fn test_diagnostics_from_multiple_language_servers(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({ "a.rs": "one two three" }))
        .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store.clone());

    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store
            .update_diagnostic_entries(
                LanguageServerId(0),
                Path::new("/dir/a.rs").to_owned(),
                None,
                None,
                vec![DiagnosticEntry {
                    range: Unclipped(PointUtf16::new(0, 0))..Unclipped(PointUtf16::new(0, 3)),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        is_primary: true,
                        message: "syntax error a1".to_string(),
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    },
                }],
                cx,
            )
            .unwrap();
        lsp_store
            .update_diagnostic_entries(
                LanguageServerId(1),
                Path::new("/dir/a.rs").to_owned(),
                None,
                None,
                vec![DiagnosticEntry {
                    range: Unclipped(PointUtf16::new(0, 0))..Unclipped(PointUtf16::new(0, 3)),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        is_primary: true,
                        message: "syntax error b1".to_string(),
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    },
                }],
                cx,
            )
            .unwrap();

        assert_eq!(
            lsp_store.diagnostic_summary(false, cx),
            DiagnosticSummary {
                error_count: 2,
                warning_count: 0,
            }
        );
    });
}

#[gpui::test]
async fn test_edits_from_lsp2_with_past_version(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let text = "
        fn a() {
            f1();
        }
        fn b() {
            f2();
        }
        fn c() {
            f3();
        }
    "
    .unindent();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.rs": text.clone(),
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();

    let mut fake_server = fake_servers.next().await.unwrap();
    let lsp_document_version = fake_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await
        .text_document
        .version;

    // Simulate editing the buffer after the language server computes some edits.
    buffer.update(cx, |buffer, cx| {
        buffer.edit(
            [(
                Point::new(0, 0)..Point::new(0, 0),
                "// above first function\n",
            )],
            None,
            cx,
        );
        buffer.edit(
            [(
                Point::new(2, 0)..Point::new(2, 0),
                "    // inside first function\n",
            )],
            None,
            cx,
        );
        buffer.edit(
            [(
                Point::new(6, 4)..Point::new(6, 4),
                "// inside second function ",
            )],
            None,
            cx,
        );

        assert_eq!(
            buffer.text(),
            "
                // above first function
                fn a() {
                    // inside first function
                    f1();
                }
                fn b() {
                    // inside second function f2();
                }
                fn c() {
                    f3();
                }
            "
            .unindent()
        );
    });

    let edits = lsp_store
        .update(cx, |lsp_store, cx| {
            lsp_store.as_local_mut().unwrap().edits_from_lsp(
                &buffer,
                vec![
                    // replace body of first function
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(3, 0)),
                        new_text: "
                            fn a() {
                                f10();
                            }
                            "
                        .unindent(),
                    },
                    // edit inside second function
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(4, 6), lsp::Position::new(4, 6)),
                        new_text: "00".into(),
                    },
                    // edit inside third function via two distinct edits
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(7, 5), lsp::Position::new(7, 5)),
                        new_text: "4000".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(7, 5), lsp::Position::new(7, 6)),
                        new_text: "".into(),
                    },
                ],
                LanguageServerId(0),
                Some(lsp_document_version),
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        for (range, new_text) in edits {
            buffer.edit([(range, new_text)], None, cx);
        }
        assert_eq!(
            buffer.text(),
            "
                // above first function
                fn a() {
                    // inside first function
                    f10();
                }
                fn b() {
                    // inside second function f200();
                }
                fn c() {
                    f4000();
                }
                "
            .unindent()
        );
    });
}

#[gpui::test]
async fn test_edits_from_lsp2_with_edits_on_adjacent_lines(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let text = "
        use a::b;
        use a::c;

        fn f() {
            b();
            c();
        }
    "
    .unindent();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.rs": text.clone(),
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();

    // Simulate the language server sending us a small edit in the form of a very large diff.
    // Rust-analyzer does this when performing a merge-imports code action.
    let edits = lsp_store
        .update(cx, |lsp_store, cx| {
            lsp_store.as_local_mut().unwrap().edits_from_lsp(
                &buffer,
                [
                    // Replace the first use statement without editing the semicolon.
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 8)),
                        new_text: "a::{b, c}".into(),
                    },
                    // Reinsert the remainder of the file between the semicolon and the final
                    // newline of the file.
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "\n\n".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "
                            fn f() {
                                b();
                                c();
                            }"
                        .unindent(),
                    },
                    // Delete everything after the first newline of the file.
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(7, 0)),
                        new_text: "".into(),
                    },
                ],
                LanguageServerId(0),
                None,
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        let edits = edits
            .into_iter()
            .map(|(range, text)| {
                (
                    range.start.to_point(buffer)..range.end.to_point(buffer),
                    text,
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            edits,
            [
                (Point::new(0, 4)..Point::new(0, 8), "a::{b, c}".into()),
                (Point::new(1, 0)..Point::new(2, 0), "".into())
            ]
        );

        for (range, new_text) in edits {
            buffer.edit([(range, new_text)], None, cx);
        }
        assert_eq!(
            buffer.text(),
            "
                use a::{b, c};

                fn f() {
                    b();
                    c();
                }
            "
            .unindent()
        );
    });
}

#[gpui::test]
async fn test_edits_from_lsp_with_replacement_followed_by_adjacent_insertion(
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx);

    let text = "Path()";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.rs": text
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();

    // Simulate the language server sending us a pair of edits at the same location,
    // with an insertion following a replacement (which violates the LSP spec).
    let edits = lsp_store
        .update(cx, |lsp_store, cx| {
            lsp_store.as_local_mut().unwrap().edits_from_lsp(
                &buffer,
                [
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                        new_text: "Path".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
                        new_text: "from path import Path\n\n\n".into(),
                    },
                ],
                LanguageServerId(0),
                None,
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        buffer.edit(edits, None, cx);
        assert_eq!(buffer.text(), "from path import Path\n\n\nPath()")
    });
}

#[gpui::test]
async fn test_invalid_edits_from_lsp2(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let text = "
        use a::b;
        use a::c;

        fn f() {
            b();
            c();
        }
    "
    .unindent();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.rs": text.clone(),
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/dir/a.rs"), cx)
        })
        .await
        .unwrap();

    // Simulate the language server sending us edits in a non-ordered fashion,
    // with ranges sometimes being inverted or pointing to invalid locations.
    let edits = lsp_store
        .update(cx, |lsp_store, cx| {
            lsp_store.as_local_mut().unwrap().edits_from_lsp(
                &buffer,
                [
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "\n\n".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 8), lsp::Position::new(0, 4)),
                        new_text: "a::{b, c}".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(99, 0)),
                        new_text: "".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "
                            fn f() {
                                b();
                                c();
                            }"
                        .unindent(),
                    },
                ],
                LanguageServerId(0),
                None,
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        let edits = edits
            .into_iter()
            .map(|(range, text)| {
                (
                    range.start.to_point(buffer)..range.end.to_point(buffer),
                    text,
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            edits,
            [
                (Point::new(0, 4)..Point::new(0, 8), "a::{b, c}".into()),
                (Point::new(1, 0)..Point::new(2, 0), "".into())
            ]
        );

        for (range, new_text) in edits {
            buffer.edit([(range, new_text)], None, cx);
        }
        assert_eq!(
            buffer.text(),
            "
                use a::{b, c};

                fn f() {
                    b();
                    c();
                }
            "
            .unindent()
        );
    });
}

fn chunks_with_diagnostics<T: ToOffset + ToPoint>(
    buffer: &Buffer,
    range: Range<T>,
) -> Vec<(String, Option<DiagnosticSeverity>)> {
    let mut chunks: Vec<(String, Option<DiagnosticSeverity>)> = Vec::new();
    for chunk in buffer.snapshot().chunks(range, true) {
        if chunks.last().map_or(false, |prev_chunk| {
            prev_chunk.1 == chunk.diagnostic_severity
        }) {
            chunks.last_mut().unwrap().0.push_str(chunk.text);
        } else {
            chunks.push((chunk.text.to_string(), chunk.diagnostic_severity));
        }
    }
    chunks
}

#[gpui::test(iterations = 10)]
async fn test_definition(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.rs": "const fn a() { A }",
            "b.rs": "const y: i32 = crate::a()",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir/b.rs").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/b.rs"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_servers.next().await.unwrap();
    fake_server.set_request_handler::<lsp::request::GotoDefinition, _, _>(|params, _| async move {
        let params = params.text_document_position_params;
        assert_eq!(
            params.text_document.uri.to_file_path().unwrap(),
            Path::new(path!("/dir/b.rs")),
        );
        assert_eq!(params.position, lsp::Position::new(0, 22));

        Ok(Some(lsp::GotoDefinitionResponse::Scalar(
            lsp::Location::new(
                lsp::Url::from_file_path(path!("/dir/a.rs")).unwrap(),
                lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
            ),
        )))
    });
    let mut definitions = project
        .update(cx, |project, cx| project.definitions(&buffer, 22, cx))
        .await
        .unwrap();

    // Assert no new language server started
    cx.executor().run_until_parked();
    assert!(fake_servers.try_next().is_err());

    assert_eq!(definitions.len(), 1);
    let definition = definitions.pop().unwrap();
    cx.update(|cx| {
        let target_buffer = definition.target.buffer.read(cx);
        assert_eq!(
            target_buffer
                .file()
                .unwrap()
                .as_local()
                .unwrap()
                .abs_path(cx),
            Path::new(path!("/dir/a.rs")),
        );
        assert_eq!(definition.target.range.to_offset(target_buffer), 9..10);
        assert_eq!(
            list_worktrees(&project, cx),
            [
                (path!("/dir/a.rs").as_ref(), false),
                (path!("/dir/b.rs").as_ref(), true)
            ],
        );

        drop(definition);
    });
    cx.update(|cx| {
        assert_eq!(
            list_worktrees(&project, cx),
            [(path!("/dir/b.rs").as_ref(), true)]
        );
    });

    fn list_worktrees<'a>(project: &'a Entity<Project>, cx: &'a App) -> Vec<(&'a Path, bool)> {
        project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| {
                let worktree = worktree.read(cx);
                (
                    worktree.as_local().unwrap().abs_path().as_ref(),
                    worktree.is_visible(),
                )
            })
            .collect::<Vec<_>>()
    }
}

#[gpui::test]
async fn test_completions_with_text_edit(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.ts": "",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.ts"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_language_servers.next().await.unwrap();

    // When text_edit exists, it takes precedence over insert_text and label
    let text = "let a = obj.fqn";
    buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
    let completions = project.update(cx, |project, cx| {
        project.completions(&buffer, text.len(), DEFAULT_COMPLETION_CONTEXT, cx)
    });

    fake_server
        .set_request_handler::<lsp::request::Completion, _, _>(|_, _| async {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "labelText".into(),
                    insert_text: Some("insertText".into()),
                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        range: lsp::Range::new(
                            lsp::Position::new(0, text.len() as u32 - 3),
                            lsp::Position::new(0, text.len() as u32),
                        ),
                        new_text: "textEditText".into(),
                    })),
                    ..Default::default()
                },
            ])))
        })
        .next()
        .await;

    let completions = completions
        .await
        .unwrap()
        .into_iter()
        .flat_map(|response| response.completions)
        .collect::<Vec<_>>();
    let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());

    assert_eq!(completions.len(), 1);
    assert_eq!(completions[0].new_text, "textEditText");
    assert_eq!(
        completions[0].replace_range.to_offset(&snapshot),
        text.len() - 3..text.len()
    );
}

#[gpui::test]
async fn test_completions_with_edit_ranges(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.ts": "",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.ts"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_language_servers.next().await.unwrap();
    let text = "let a = obj.fqn";

    // Test 1: When text_edit is None but insert_text exists with default edit_range
    {
        buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
        let completions = project.update(cx, |project, cx| {
            project.completions(&buffer, text.len(), DEFAULT_COMPLETION_CONTEXT, cx)
        });

        fake_server
            .set_request_handler::<lsp::request::Completion, _, _>(|_, _| async {
                Ok(Some(lsp::CompletionResponse::List(lsp::CompletionList {
                    is_incomplete: false,
                    item_defaults: Some(lsp::CompletionListItemDefaults {
                        edit_range: Some(lsp::CompletionListItemDefaultsEditRange::Range(
                            lsp::Range::new(
                                lsp::Position::new(0, text.len() as u32 - 3),
                                lsp::Position::new(0, text.len() as u32),
                            ),
                        )),
                        ..Default::default()
                    }),
                    items: vec![lsp::CompletionItem {
                        label: "labelText".into(),
                        insert_text: Some("insertText".into()),
                        text_edit: None,
                        ..Default::default()
                    }],
                })))
            })
            .next()
            .await;

        let completions = completions
            .await
            .unwrap()
            .into_iter()
            .flat_map(|response| response.completions)
            .collect::<Vec<_>>();
        let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());

        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].new_text, "insertText");
        assert_eq!(
            completions[0].replace_range.to_offset(&snapshot),
            text.len() - 3..text.len()
        );
    }

    // Test 2: When both text_edit and insert_text are None with default edit_range
    {
        buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
        let completions = project.update(cx, |project, cx| {
            project.completions(&buffer, text.len(), DEFAULT_COMPLETION_CONTEXT, cx)
        });

        fake_server
            .set_request_handler::<lsp::request::Completion, _, _>(|_, _| async {
                Ok(Some(lsp::CompletionResponse::List(lsp::CompletionList {
                    is_incomplete: false,
                    item_defaults: Some(lsp::CompletionListItemDefaults {
                        edit_range: Some(lsp::CompletionListItemDefaultsEditRange::Range(
                            lsp::Range::new(
                                lsp::Position::new(0, text.len() as u32 - 3),
                                lsp::Position::new(0, text.len() as u32),
                            ),
                        )),
                        ..Default::default()
                    }),
                    items: vec![lsp::CompletionItem {
                        label: "labelText".into(),
                        insert_text: None,
                        text_edit: None,
                        ..Default::default()
                    }],
                })))
            })
            .next()
            .await;

        let completions = completions
            .await
            .unwrap()
            .into_iter()
            .flat_map(|response| response.completions)
            .collect::<Vec<_>>();
        let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());

        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].new_text, "labelText");
        assert_eq!(
            completions[0].replace_range.to_offset(&snapshot),
            text.len() - 3..text.len()
        );
    }
}

#[gpui::test]
async fn test_completions_without_edit_ranges(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.ts": "",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.ts"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_language_servers.next().await.unwrap();

    // Test 1: When text_edit is None but insert_text exists (no edit_range in defaults)
    let text = "let a = b.fqn";
    buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
    let completions = project.update(cx, |project, cx| {
        project.completions(&buffer, text.len(), DEFAULT_COMPLETION_CONTEXT, cx)
    });

    fake_server
        .set_request_handler::<lsp::request::Completion, _, _>(|_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "fullyQualifiedName?".into(),
                    insert_text: Some("fullyQualifiedName".into()),
                    ..Default::default()
                },
            ])))
        })
        .next()
        .await;
    let completions = completions
        .await
        .unwrap()
        .into_iter()
        .flat_map(|response| response.completions)
        .collect::<Vec<_>>();
    let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
    assert_eq!(completions.len(), 1);
    assert_eq!(completions[0].new_text, "fullyQualifiedName");
    assert_eq!(
        completions[0].replace_range.to_offset(&snapshot),
        text.len() - 3..text.len()
    );

    // Test 2: When both text_edit and insert_text are None (no edit_range in defaults)
    let text = "let a = \"atoms/cmp\"";
    buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
    let completions = project.update(cx, |project, cx| {
        project.completions(&buffer, text.len() - 1, DEFAULT_COMPLETION_CONTEXT, cx)
    });

    fake_server
        .set_request_handler::<lsp::request::Completion, _, _>(|_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "component".into(),
                    ..Default::default()
                },
            ])))
        })
        .next()
        .await;
    let completions = completions
        .await
        .unwrap()
        .into_iter()
        .flat_map(|response| response.completions)
        .collect::<Vec<_>>();
    let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
    assert_eq!(completions.len(), 1);
    assert_eq!(completions[0].new_text, "component");
    assert_eq!(
        completions[0].replace_range.to_offset(&snapshot),
        text.len() - 4..text.len() - 1
    );
}

#[gpui::test]
async fn test_completions_with_carriage_returns(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.ts": "",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.ts"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_language_servers.next().await.unwrap();

    let text = "let a = b.fqn";
    buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
    let completions = project.update(cx, |project, cx| {
        project.completions(&buffer, text.len(), DEFAULT_COMPLETION_CONTEXT, cx)
    });

    fake_server
        .set_request_handler::<lsp::request::Completion, _, _>(|_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "fullyQualifiedName?".into(),
                    insert_text: Some("fully\rQualified\r\nName".into()),
                    ..Default::default()
                },
            ])))
        })
        .next()
        .await;
    let completions = completions
        .await
        .unwrap()
        .into_iter()
        .flat_map(|response| response.completions)
        .collect::<Vec<_>>();
    assert_eq!(completions.len(), 1);
    assert_eq!(completions[0].new_text, "fully\nQualified\nName");
}

#[gpui::test(iterations = 10)]
async fn test_apply_code_actions_with_commands(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.ts": "a",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                code_action_provider: Some(lsp::CodeActionProviderCapability::Options(
                    lsp::CodeActionOptions {
                        resolve_provider: Some(true),
                        ..lsp::CodeActionOptions::default()
                    },
                )),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["_the/command".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.ts"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_language_servers.next().await.unwrap();

    // Language server returns code actions that contain commands, and not edits.
    let actions = project.update(cx, |project, cx| {
        project.code_actions(&buffer, 0..0, None, cx)
    });
    fake_server
        .set_request_handler::<lsp::request::CodeActionRequest, _, _>(|_, _| async move {
            Ok(Some(vec![
                lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                    title: "The code action".into(),
                    data: Some(serde_json::json!({
                        "command": "_the/command",
                    })),
                    ..lsp::CodeAction::default()
                }),
                lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                    title: "two".into(),
                    ..lsp::CodeAction::default()
                }),
            ]))
        })
        .next()
        .await;

    let action = actions.await.unwrap()[0].clone();
    let apply = project.update(cx, |project, cx| {
        project.apply_code_action(buffer.clone(), action, true, cx)
    });

    // Resolving the code action does not populate its edits. In absence of
    // edits, we must execute the given command.
    fake_server.set_request_handler::<lsp::request::CodeActionResolveRequest, _, _>(
        |mut action, _| async move {
            if action.data.is_some() {
                action.command = Some(lsp::Command {
                    title: "The command".into(),
                    command: "_the/command".into(),
                    arguments: Some(vec![json!("the-argument")]),
                });
            }
            Ok(action)
        },
    );

    // While executing the command, the language server sends the editor
    // a `workspaceEdit` request.
    fake_server
        .set_request_handler::<lsp::request::ExecuteCommand, _, _>({
            let fake = fake_server.clone();
            move |params, _| {
                assert_eq!(params.command, "_the/command");
                let fake = fake.clone();
                async move {
                    fake.server
                        .request::<lsp::request::ApplyWorkspaceEdit>(
                            lsp::ApplyWorkspaceEditParams {
                                label: None,
                                edit: lsp::WorkspaceEdit {
                                    changes: Some(
                                        [(
                                            lsp::Url::from_file_path(path!("/dir/a.ts")).unwrap(),
                                            vec![lsp::TextEdit {
                                                range: lsp::Range::new(
                                                    lsp::Position::new(0, 0),
                                                    lsp::Position::new(0, 0),
                                                ),
                                                new_text: "X".into(),
                                            }],
                                        )]
                                        .into_iter()
                                        .collect(),
                                    ),
                                    ..Default::default()
                                },
                            },
                        )
                        .await
                        .into_response()
                        .unwrap();
                    Ok(Some(json!(null)))
                }
            }
        })
        .next()
        .await;

    // Applying the code action returns a project transaction containing the edits
    // sent by the language server in its `workspaceEdit` request.
    let transaction = apply.await.unwrap();
    assert!(transaction.0.contains_key(&buffer));
    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "Xa");
        buffer.undo(cx);
        assert_eq!(buffer.text(), "a");
    });
}

#[gpui::test(iterations = 10)]
async fn test_save_file(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "file1": "the old contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/file1"), cx))
        .await
        .unwrap();
    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "the old contents");
        buffer.edit([(0..0, "a line of text.\n".repeat(10 * 1024))], None, cx);
    });

    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .await
        .unwrap();

    let new_text = fs
        .load(Path::new(path!("/dir/file1")))
        .await
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(new_text, buffer.update(cx, |buffer, _| buffer.text()));
}

#[gpui::test(iterations = 10)]
async fn test_save_file_spawns_language_server(cx: &mut gpui::TestAppContext) {
    // Issue: #24349
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), json!({})).await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    language_registry.add(rust_lang());
    let mut fake_rust_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-rust-language-server",
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), "::".to_string()]),
                    ..Default::default()
                }),
                text_document_sync: Some(lsp::TextDocumentSyncCapability::Options(
                    lsp::TextDocumentSyncOptions {
                        save: Some(lsp::TextDocumentSyncSaveOptions::Supported(true)),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let buffer = project
        .update(cx, |this, cx| this.create_buffer(cx))
        .unwrap()
        .await;
    project.update(cx, |this, cx| {
        this.register_buffer_with_language_servers(&buffer, cx);
        buffer.update(cx, |buffer, cx| {
            assert!(!this.has_language_servers_for(buffer, cx));
        })
    });

    project
        .update(cx, |this, cx| {
            let worktree_id = this.worktrees(cx).next().unwrap().read(cx).id();
            this.save_buffer_as(
                buffer.clone(),
                ProjectPath {
                    worktree_id,
                    path: Arc::from("file.rs".as_ref()),
                },
                cx,
            )
        })
        .await
        .unwrap();
    // A server is started up, and it is notified about Rust files.
    let mut fake_rust_server = fake_rust_servers.next().await.unwrap();
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path(path!("/dir/file.rs")).unwrap(),
            version: 0,
            text: "".to_string(),
            language_id: "rust".to_string(),
        }
    );

    project.update(cx, |this, cx| {
        buffer.update(cx, |buffer, cx| {
            assert!(this.has_language_servers_for(buffer, cx));
        })
    });
}

#[gpui::test(iterations = 30)]
async fn test_file_changes_multiple_times_on_disk(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "file1": "the original contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let worktree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/file1"), cx))
        .await
        .unwrap();

    // Simulate buffer diffs being slow, so that they don't complete before
    // the next file change occurs.
    cx.executor().deprioritize(*language::BUFFER_DIFF_TASK);

    // Change the buffer's file on disk, and then wait for the file change
    // to be detected by the worktree, so that the buffer starts reloading.
    fs.save(
        path!("/dir/file1").as_ref(),
        &"the first contents".into(),
        Default::default(),
    )
    .await
    .unwrap();
    worktree.next_event(cx).await;

    // Change the buffer's file again. Depending on the random seed, the
    // previous file change may still be in progress.
    fs.save(
        path!("/dir/file1").as_ref(),
        &"the second contents".into(),
        Default::default(),
    )
    .await
    .unwrap();
    worktree.next_event(cx).await;

    cx.executor().run_until_parked();
    let on_disk_text = fs.load(Path::new(path!("/dir/file1"))).await.unwrap();
    buffer.read_with(cx, |buffer, _| {
        assert_eq!(buffer.text(), on_disk_text);
        assert!(!buffer.is_dirty(), "buffer should not be dirty");
        assert!(!buffer.has_conflict(), "buffer should not be dirty");
    });
}

#[gpui::test(iterations = 30)]
async fn test_edit_buffer_while_it_reloads(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "file1": "the original contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let worktree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/file1"), cx))
        .await
        .unwrap();

    // Simulate buffer diffs being slow, so that they don't complete before
    // the next file change occurs.
    cx.executor().deprioritize(*language::BUFFER_DIFF_TASK);

    // Change the buffer's file on disk, and then wait for the file change
    // to be detected by the worktree, so that the buffer starts reloading.
    fs.save(
        path!("/dir/file1").as_ref(),
        &"the first contents".into(),
        Default::default(),
    )
    .await
    .unwrap();
    worktree.next_event(cx).await;

    cx.executor()
        .spawn(cx.executor().simulate_random_delay())
        .await;

    // Perform a noop edit, causing the buffer's version to increase.
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, " ")], None, cx);
        buffer.undo(cx);
    });

    cx.executor().run_until_parked();
    let on_disk_text = fs.load(Path::new(path!("/dir/file1"))).await.unwrap();
    buffer.read_with(cx, |buffer, _| {
        let buffer_text = buffer.text();
        if buffer_text == on_disk_text {
            assert!(
                !buffer.is_dirty() && !buffer.has_conflict(),
                "buffer shouldn't be dirty. text: {buffer_text:?}, disk text: {on_disk_text:?}",
            );
        }
        // If the file change occurred while the buffer was processing the first
        // change, the buffer will be in a conflicting state.
        else {
            assert!(buffer.is_dirty(), "buffer should report that it is dirty. text: {buffer_text:?}, disk text: {on_disk_text:?}");
            assert!(buffer.has_conflict(), "buffer should report that it is dirty. text: {buffer_text:?}, disk text: {on_disk_text:?}");
        }
    });
}

#[gpui::test]
async fn test_save_in_single_file_worktree(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "file1": "the old contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir/file1").as_ref()], cx).await;
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/file1"), cx))
        .await
        .unwrap();
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "a line of text.\n".repeat(10 * 1024))], None, cx);
    });

    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .await
        .unwrap();

    let new_text = fs
        .load(Path::new(path!("/dir/file1")))
        .await
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(new_text, buffer.update(cx, |buffer, _| buffer.text()));
}

#[gpui::test]
async fn test_save_as(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({})).await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    let languages = project.update(cx, |project, _| project.languages().clone());
    languages.add(rust_lang());

    let buffer = project.update(cx, |project, cx| project.create_local_buffer("", None, cx));
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "abc")], None, cx);
        assert!(buffer.is_dirty());
        assert!(!buffer.has_conflict());
        assert_eq!(buffer.language().unwrap().name(), "Plain Text".into());
    });
    project
        .update(cx, |project, cx| {
            let worktree_id = project.worktrees(cx).next().unwrap().read(cx).id();
            let path = ProjectPath {
                worktree_id,
                path: Arc::from(Path::new("file1.rs")),
            };
            project.save_buffer_as(buffer.clone(), path, cx)
        })
        .await
        .unwrap();
    assert_eq!(fs.load(Path::new("/dir/file1.rs")).await.unwrap(), "abc");

    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, cx| {
        assert_eq!(
            buffer.file().unwrap().full_path(cx),
            Path::new("dir/file1.rs")
        );
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());
        assert_eq!(buffer.language().unwrap().name(), "Rust".into());
    });

    let opened_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/file1.rs", cx)
        })
        .await
        .unwrap();
    assert_eq!(opened_buffer, buffer);
}

#[gpui::test(retries = 5)]
async fn test_rescan_and_remote_updates(cx: &mut gpui::TestAppContext) {
    use worktree::WorktreeModelHandle as _;

    init_test(cx);
    cx.executor().allow_parking();

    let dir = TempTree::new(json!({
        "a": {
            "file1": "",
            "file2": "",
            "file3": "",
        },
        "b": {
            "c": {
                "file4": "",
                "file5": "",
            }
        }
    }));

    let project = Project::test(Arc::new(RealFs::new(None, cx.executor())), [dir.path()], cx).await;

    let buffer_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
        let buffer = project.update(cx, |p, cx| p.open_local_buffer(dir.path().join(path), cx));
        async move { buffer.await.unwrap() }
    };
    let id_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
        project.update(cx, |project, cx| {
            let tree = project.worktrees(cx).next().unwrap();
            tree.read(cx)
                .entry_for_path(path)
                .unwrap_or_else(|| panic!("no entry for path {}", path))
                .id
        })
    };

    let buffer2 = buffer_for_path("a/file2", cx).await;
    let buffer3 = buffer_for_path("a/file3", cx).await;
    let buffer4 = buffer_for_path("b/c/file4", cx).await;
    let buffer5 = buffer_for_path("b/c/file5", cx).await;

    let file2_id = id_for_path("a/file2", cx);
    let file3_id = id_for_path("a/file3", cx);
    let file4_id = id_for_path("b/c/file4", cx);

    // Create a remote copy of this worktree.
    let tree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
    let metadata = tree.update(cx, |tree, _| tree.metadata_proto());

    let updates = Arc::new(Mutex::new(Vec::new()));
    tree.update(cx, |tree, cx| {
        let updates = updates.clone();
        tree.observe_updates(0, cx, move |update| {
            updates.lock().push(update);
            async { true }
        });
    });

    let remote =
        cx.update(|cx| Worktree::remote(0, 1, metadata, project.read(cx).client().into(), cx));

    cx.executor().run_until_parked();

    cx.update(|cx| {
        assert!(!buffer2.read(cx).is_dirty());
        assert!(!buffer3.read(cx).is_dirty());
        assert!(!buffer4.read(cx).is_dirty());
        assert!(!buffer5.read(cx).is_dirty());
    });

    // Rename and delete files and directories.
    tree.flush_fs_events(cx).await;
    std::fs::rename(dir.path().join("a/file3"), dir.path().join("b/c/file3")).unwrap();
    std::fs::remove_file(dir.path().join("b/c/file5")).unwrap();
    std::fs::rename(dir.path().join("b/c"), dir.path().join("d")).unwrap();
    std::fs::rename(dir.path().join("a/file2"), dir.path().join("a/file2.new")).unwrap();
    tree.flush_fs_events(cx).await;

    cx.update(|app| {
        assert_eq!(
            tree.read(app)
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            vec![
                "a",
                path!("a/file1"),
                path!("a/file2.new"),
                "b",
                "d",
                path!("d/file3"),
                path!("d/file4"),
            ]
        );
    });

    assert_eq!(id_for_path("a/file2.new", cx), file2_id);
    assert_eq!(id_for_path("d/file3", cx), file3_id);
    assert_eq!(id_for_path("d/file4", cx), file4_id);

    cx.update(|cx| {
        assert_eq!(
            buffer2.read(cx).file().unwrap().path().as_ref(),
            Path::new("a/file2.new")
        );
        assert_eq!(
            buffer3.read(cx).file().unwrap().path().as_ref(),
            Path::new("d/file3")
        );
        assert_eq!(
            buffer4.read(cx).file().unwrap().path().as_ref(),
            Path::new("d/file4")
        );
        assert_eq!(
            buffer5.read(cx).file().unwrap().path().as_ref(),
            Path::new("b/c/file5")
        );

        assert_matches!(
            buffer2.read(cx).file().unwrap().disk_state(),
            DiskState::Present { .. }
        );
        assert_matches!(
            buffer3.read(cx).file().unwrap().disk_state(),
            DiskState::Present { .. }
        );
        assert_matches!(
            buffer4.read(cx).file().unwrap().disk_state(),
            DiskState::Present { .. }
        );
        assert_eq!(
            buffer5.read(cx).file().unwrap().disk_state(),
            DiskState::Deleted
        );
    });

    // Update the remote worktree. Check that it becomes consistent with the
    // local worktree.
    cx.executor().run_until_parked();

    remote.update(cx, |remote, _| {
        for update in updates.lock().drain(..) {
            remote.as_remote_mut().unwrap().update_from_remote(update);
        }
    });
    cx.executor().run_until_parked();
    remote.update(cx, |remote, _| {
        assert_eq!(
            remote
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            vec![
                "a",
                path!("a/file1"),
                path!("a/file2.new"),
                "b",
                "d",
                path!("d/file3"),
                path!("d/file4"),
            ]
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_buffer_identity_across_renames(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a": {
                "file1": "",
            }
        }),
    )
    .await;

    let project = Project::test(fs, [Path::new(path!("/dir"))], cx).await;
    let tree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
    let tree_id = tree.update(cx, |tree, _| tree.id());

    let id_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
        project.update(cx, |project, cx| {
            let tree = project.worktrees(cx).next().unwrap();
            tree.read(cx)
                .entry_for_path(path)
                .unwrap_or_else(|| panic!("no entry for path {}", path))
                .id
        })
    };

    let dir_id = id_for_path("a", cx);
    let file_id = id_for_path("a/file1", cx);
    let buffer = project
        .update(cx, |p, cx| p.open_buffer((tree_id, "a/file1"), cx))
        .await
        .unwrap();
    buffer.update(cx, |buffer, _| assert!(!buffer.is_dirty()));

    project
        .update(cx, |project, cx| {
            project.rename_entry(dir_id, Path::new("b"), cx)
        })
        .unwrap()
        .await
        .to_included()
        .unwrap();
    cx.executor().run_until_parked();

    assert_eq!(id_for_path("b", cx), dir_id);
    assert_eq!(id_for_path("b/file1", cx), file_id);
    buffer.update(cx, |buffer, _| assert!(!buffer.is_dirty()));
}

#[gpui::test]
async fn test_buffer_deduping(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.txt": "a-contents",
            "b.txt": "b-contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    // Spawn multiple tasks to open paths, repeating some paths.
    let (buffer_a_1, buffer_b, buffer_a_2) = project.update(cx, |p, cx| {
        (
            p.open_local_buffer("/dir/a.txt", cx),
            p.open_local_buffer("/dir/b.txt", cx),
            p.open_local_buffer("/dir/a.txt", cx),
        )
    });

    let buffer_a_1 = buffer_a_1.await.unwrap();
    let buffer_a_2 = buffer_a_2.await.unwrap();
    let buffer_b = buffer_b.await.unwrap();
    assert_eq!(buffer_a_1.update(cx, |b, _| b.text()), "a-contents");
    assert_eq!(buffer_b.update(cx, |b, _| b.text()), "b-contents");

    // There is only one buffer per path.
    let buffer_a_id = buffer_a_1.entity_id();
    assert_eq!(buffer_a_2.entity_id(), buffer_a_id);

    // Open the same path again while it is still open.
    drop(buffer_a_1);
    let buffer_a_3 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.txt", cx))
        .await
        .unwrap();

    // There's still only one buffer per path.
    assert_eq!(buffer_a_3.entity_id(), buffer_a_id);
}

#[gpui::test]
async fn test_buffer_is_dirty(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "file1": "abc",
            "file2": "def",
            "file3": "ghi",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

    let buffer1 = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/file1"), cx))
        .await
        .unwrap();
    let events = Arc::new(Mutex::new(Vec::new()));

    // initially, the buffer isn't dirty.
    buffer1.update(cx, |buffer, cx| {
        cx.subscribe(&buffer1, {
            let events = events.clone();
            move |_, _, event, _| match event {
                BufferEvent::Operation { .. } => {}
                _ => events.lock().push(event.clone()),
            }
        })
        .detach();

        assert!(!buffer.is_dirty());
        assert!(events.lock().is_empty());

        buffer.edit([(1..2, "")], None, cx);
    });

    // after the first edit, the buffer is dirty, and emits a dirtied event.
    buffer1.update(cx, |buffer, cx| {
        assert!(buffer.text() == "ac");
        assert!(buffer.is_dirty());
        assert_eq!(
            *events.lock(),
            &[
                language::BufferEvent::Edited,
                language::BufferEvent::DirtyChanged
            ]
        );
        events.lock().clear();
        buffer.did_save(
            buffer.version(),
            buffer.file().unwrap().disk_state().mtime(),
            cx,
        );
    });

    // after saving, the buffer is not dirty, and emits a saved event.
    buffer1.update(cx, |buffer, cx| {
        assert!(!buffer.is_dirty());
        assert_eq!(*events.lock(), &[language::BufferEvent::Saved]);
        events.lock().clear();

        buffer.edit([(1..1, "B")], None, cx);
        buffer.edit([(2..2, "D")], None, cx);
    });

    // after editing again, the buffer is dirty, and emits another dirty event.
    buffer1.update(cx, |buffer, cx| {
        assert!(buffer.text() == "aBDc");
        assert!(buffer.is_dirty());
        assert_eq!(
            *events.lock(),
            &[
                language::BufferEvent::Edited,
                language::BufferEvent::DirtyChanged,
                language::BufferEvent::Edited,
            ],
        );
        events.lock().clear();

        // After restoring the buffer to its previously-saved state,
        // the buffer is not considered dirty anymore.
        buffer.edit([(1..3, "")], None, cx);
        assert!(buffer.text() == "ac");
        assert!(!buffer.is_dirty());
    });

    assert_eq!(
        *events.lock(),
        &[
            language::BufferEvent::Edited,
            language::BufferEvent::DirtyChanged
        ]
    );

    // When a file is deleted, it is not considered dirty.
    let events = Arc::new(Mutex::new(Vec::new()));
    let buffer2 = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/file2"), cx))
        .await
        .unwrap();
    buffer2.update(cx, |_, cx| {
        cx.subscribe(&buffer2, {
            let events = events.clone();
            move |_, _, event, _| match event {
                BufferEvent::Operation { .. } => {}
                _ => events.lock().push(event.clone()),
            }
        })
        .detach();
    });

    fs.remove_file(path!("/dir/file2").as_ref(), Default::default())
        .await
        .unwrap();
    cx.executor().run_until_parked();
    buffer2.update(cx, |buffer, _| assert!(!buffer.is_dirty()));
    assert_eq!(
        mem::take(&mut *events.lock()),
        &[language::BufferEvent::FileHandleChanged]
    );

    // Buffer becomes dirty when edited.
    buffer2.update(cx, |buffer, cx| {
        buffer.edit([(2..3, "")], None, cx);
        assert_eq!(buffer.is_dirty(), true);
    });
    assert_eq!(
        mem::take(&mut *events.lock()),
        &[
            language::BufferEvent::Edited,
            language::BufferEvent::DirtyChanged
        ]
    );

    // Buffer becomes clean again when all of its content is removed, because
    // the file was deleted.
    buffer2.update(cx, |buffer, cx| {
        buffer.edit([(0..2, "")], None, cx);
        assert_eq!(buffer.is_empty(), true);
        assert_eq!(buffer.is_dirty(), false);
    });
    assert_eq!(
        *events.lock(),
        &[
            language::BufferEvent::Edited,
            language::BufferEvent::DirtyChanged
        ]
    );

    // When a file is already dirty when deleted, we don't emit a Dirtied event.
    let events = Arc::new(Mutex::new(Vec::new()));
    let buffer3 = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/file3"), cx))
        .await
        .unwrap();
    buffer3.update(cx, |_, cx| {
        cx.subscribe(&buffer3, {
            let events = events.clone();
            move |_, _, event, _| match event {
                BufferEvent::Operation { .. } => {}
                _ => events.lock().push(event.clone()),
            }
        })
        .detach();
    });

    buffer3.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "x")], None, cx);
    });
    events.lock().clear();
    fs.remove_file(path!("/dir/file3").as_ref(), Default::default())
        .await
        .unwrap();
    cx.executor().run_until_parked();
    assert_eq!(*events.lock(), &[language::BufferEvent::FileHandleChanged]);
    cx.update(|cx| assert!(buffer3.read(cx).is_dirty()));
}

#[gpui::test]
async fn test_buffer_file_changes_on_disk(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let (initial_contents, initial_offsets) =
        marked_text_offsets("one twoˇ\nthree ˇfourˇ five\nsixˇ seven\n");
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "the-file": initial_contents,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/the-file"), cx))
        .await
        .unwrap();

    let anchors = initial_offsets
        .iter()
        .map(|offset| buffer.update(cx, |b, _| b.anchor_before(offset)))
        .collect::<Vec<_>>();

    // Change the file on disk, adding two new lines of text, and removing
    // one line.
    buffer.update(cx, |buffer, _| {
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });

    let (new_contents, new_offsets) =
        marked_text_offsets("oneˇ\nthree ˇFOURˇ five\nsixtyˇ seven\n");
    fs.save(
        path!("/dir/the-file").as_ref(),
        &new_contents.as_str().into(),
        LineEnding::Unix,
    )
    .await
    .unwrap();

    // Because the buffer was not modified, it is reloaded from disk. Its
    // contents are edited according to the diff between the old and new
    // file contents.
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), new_contents);
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());

        let anchor_offsets = anchors
            .iter()
            .map(|anchor| anchor.to_offset(&*buffer))
            .collect::<Vec<_>>();
        assert_eq!(anchor_offsets, new_offsets);
    });

    // Modify the buffer
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, " ")], None, cx);
        assert!(buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });

    // Change the file on disk again, adding blank lines to the beginning.
    fs.save(
        path!("/dir/the-file").as_ref(),
        &"\n\n\nAAAA\naaa\nBB\nbbbbb\n".into(),
        LineEnding::Unix,
    )
    .await
    .unwrap();

    // Because the buffer is modified, it doesn't reload from disk, but is
    // marked as having a conflict.
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), " ".to_string() + &new_contents);
        assert!(buffer.has_conflict());
    });
}

#[gpui::test]
async fn test_buffer_line_endings(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "file1": "a\nb\nc\n",
            "file2": "one\r\ntwo\r\nthree\r\n",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let buffer1 = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/file1"), cx))
        .await
        .unwrap();
    let buffer2 = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/file2"), cx))
        .await
        .unwrap();

    buffer1.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), "a\nb\nc\n");
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
    });
    buffer2.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), "one\ntwo\nthree\n");
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
    });

    // Change a file's line endings on disk from unix to windows. The buffer's
    // state updates correctly.
    fs.save(
        path!("/dir/file1").as_ref(),
        &"aaa\nb\nc\n".into(),
        LineEnding::Windows,
    )
    .await
    .unwrap();
    cx.executor().run_until_parked();
    buffer1.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), "aaa\nb\nc\n");
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
    });

    // Save a file with windows line endings. The file is written correctly.
    buffer2.update(cx, |buffer, cx| {
        buffer.set_text("one\ntwo\nthree\nfour\n", cx);
    });
    project
        .update(cx, |project, cx| project.save_buffer(buffer2, cx))
        .await
        .unwrap();
    assert_eq!(
        fs.load(path!("/dir/file2").as_ref()).await.unwrap(),
        "one\r\ntwo\r\nthree\r\nfour\r\n",
    );
}

#[gpui::test]
async fn test_grouped_diagnostics(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.rs": "
                fn foo(mut v: Vec<usize>) {
                    for x in &v {
                        v.push(1);
                    }
                }
            "
            .unindent(),
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer(path!("/dir/a.rs"), cx))
        .await
        .unwrap();

    let buffer_uri = Url::from_file_path(path!("/dir/a.rs")).unwrap();
    let message = lsp::PublishDiagnosticsParams {
        uri: buffer_uri.clone(),
        diagnostics: vec![
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                severity: Some(DiagnosticSeverity::WARNING),
                message: "error 1".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        uri: buffer_uri.clone(),
                        range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                    },
                    message: "error 1 hint 1".to_string(),
                }]),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                severity: Some(DiagnosticSeverity::HINT),
                message: "error 1 hint 1".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        uri: buffer_uri.clone(),
                        range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                    },
                    message: "original diagnostic".to_string(),
                }]),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 17)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "error 2".to_string(),
                related_information: Some(vec![
                    lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 13),
                                lsp::Position::new(1, 15),
                            ),
                        },
                        message: "error 2 hint 1".to_string(),
                    },
                    lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 13),
                                lsp::Position::new(1, 15),
                            ),
                        },
                        message: "error 2 hint 2".to_string(),
                    },
                ]),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 13), lsp::Position::new(1, 15)),
                severity: Some(DiagnosticSeverity::HINT),
                message: "error 2 hint 1".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        uri: buffer_uri.clone(),
                        range: lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 17)),
                    },
                    message: "original diagnostic".to_string(),
                }]),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 13), lsp::Position::new(1, 15)),
                severity: Some(DiagnosticSeverity::HINT),
                message: "error 2 hint 2".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        uri: buffer_uri,
                        range: lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 17)),
                    },
                    message: "original diagnostic".to_string(),
                }]),
                ..Default::default()
            },
        ],
        version: None,
    };

    lsp_store
        .update(cx, |lsp_store, cx| {
            lsp_store.update_diagnostics(
                LanguageServerId(0),
                message,
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
        })
        .unwrap();
    let buffer = buffer.update(cx, |buffer, _| buffer.snapshot());

    assert_eq!(
        buffer
            .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
            .collect::<Vec<_>>(),
        &[
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::WARNING,
                    message: "error 1".to_string(),
                    group_id: 1,
                    is_primary: true,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 1 hint 1".to_string(),
                    group_id: 1,
                    is_primary: false,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 1".to_string(),
                    group_id: 0,
                    is_primary: false,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 2".to_string(),
                    group_id: 0,
                    is_primary: false,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(2, 8)..Point::new(2, 17),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::ERROR,
                    message: "error 2".to_string(),
                    group_id: 0,
                    is_primary: true,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            }
        ]
    );

    assert_eq!(
        buffer.diagnostic_group::<Point>(0).collect::<Vec<_>>(),
        &[
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 1".to_string(),
                    group_id: 0,
                    is_primary: false,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 2".to_string(),
                    group_id: 0,
                    is_primary: false,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(2, 8)..Point::new(2, 17),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::ERROR,
                    message: "error 2".to_string(),
                    group_id: 0,
                    is_primary: true,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            }
        ]
    );

    assert_eq!(
        buffer.diagnostic_group::<Point>(1).collect::<Vec<_>>(),
        &[
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::WARNING,
                    message: "error 1".to_string(),
                    group_id: 1,
                    is_primary: true,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 1 hint 1".to_string(),
                    group_id: 1,
                    is_primary: false,
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                }
            },
        ]
    );
}

#[gpui::test]
async fn test_lsp_rename_notifications(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "one.rs": "const ONE: usize = 1;",
            "two": {
                "two.rs": "const TWO: usize = one::ONE + one::ONE;"
            }

        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let watched_paths = lsp::FileOperationRegistrationOptions {
        filters: vec![
            FileOperationFilter {
                scheme: Some("file".to_owned()),
                pattern: lsp::FileOperationPattern {
                    glob: "**/*.rs".to_owned(),
                    matches: Some(lsp::FileOperationPatternKind::File),
                    options: None,
                },
            },
            FileOperationFilter {
                scheme: Some("file".to_owned()),
                pattern: lsp::FileOperationPattern {
                    glob: "**/**".to_owned(),
                    matches: Some(lsp::FileOperationPatternKind::Folder),
                    options: None,
                },
            },
        ],
    };
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                workspace: Some(lsp::WorkspaceServerCapabilities {
                    workspace_folders: None,
                    file_operations: Some(lsp::WorkspaceFileOperationsServerCapabilities {
                        did_rename: Some(watched_paths.clone()),
                        will_rename: Some(watched_paths),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let _ = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/one.rs"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_servers.next().await.unwrap();
    let response = project.update(cx, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap();
        let entry = worktree.read(cx).entry_for_path("one.rs").unwrap();
        project.rename_entry(entry.id, "three.rs".as_ref(), cx)
    });
    let expected_edit = lsp::WorkspaceEdit {
        changes: None,
        document_changes: Some(DocumentChanges::Edits({
            vec![TextDocumentEdit {
                edits: vec![lsp::Edit::Plain(lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 1,
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 3,
                        },
                    },
                    new_text: "This is not a drill".to_owned(),
                })],
                text_document: lsp::OptionalVersionedTextDocumentIdentifier {
                    uri: Url::from_str(uri!("file:///dir/two/two.rs")).unwrap(),
                    version: Some(1337),
                },
            }]
        })),
        change_annotations: None,
    };
    let resolved_workspace_edit = Arc::new(OnceLock::new());
    fake_server
        .set_request_handler::<WillRenameFiles, _, _>({
            let resolved_workspace_edit = resolved_workspace_edit.clone();
            let expected_edit = expected_edit.clone();
            move |params, _| {
                let resolved_workspace_edit = resolved_workspace_edit.clone();
                let expected_edit = expected_edit.clone();
                async move {
                    assert_eq!(params.files.len(), 1);
                    assert_eq!(params.files[0].old_uri, uri!("file:///dir/one.rs"));
                    assert_eq!(params.files[0].new_uri, uri!("file:///dir/three.rs"));
                    resolved_workspace_edit.set(expected_edit.clone()).unwrap();
                    Ok(Some(expected_edit))
                }
            }
        })
        .next()
        .await
        .unwrap();
    let _ = response.await.unwrap();
    fake_server
        .handle_notification::<DidRenameFiles, _>(|params, _| {
            assert_eq!(params.files.len(), 1);
            assert_eq!(params.files[0].old_uri, uri!("file:///dir/one.rs"));
            assert_eq!(params.files[0].new_uri, uri!("file:///dir/three.rs"));
        })
        .next()
        .await
        .unwrap();
    assert_eq!(resolved_workspace_edit.get(), Some(&expected_edit));
}

#[gpui::test]
async fn test_rename(cx: &mut gpui::TestAppContext) {
    // hi
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "one.rs": "const ONE: usize = 1;",
            "two.rs": "const TWO: usize = one::ONE + one::ONE;"
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/one.rs"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_servers.next().await.unwrap();

    let response = project.update(cx, |project, cx| {
        project.prepare_rename(buffer.clone(), 7, cx)
    });
    fake_server
        .set_request_handler::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri.as_str(),
                uri!("file:///dir/one.rs")
            );
            assert_eq!(params.position, lsp::Position::new(0, 7));
            Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                lsp::Position::new(0, 6),
                lsp::Position::new(0, 9),
            ))))
        })
        .next()
        .await
        .unwrap();
    let response = response.await.unwrap();
    let PrepareRenameResponse::Success(range) = response else {
        panic!("{:?}", response);
    };
    let range = buffer.update(cx, |buffer, _| range.to_offset(buffer));
    assert_eq!(range, 6..9);

    let response = project.update(cx, |project, cx| {
        project.perform_rename(buffer.clone(), 7, "THREE".to_string(), cx)
    });
    fake_server
        .set_request_handler::<lsp::request::Rename, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri.as_str(),
                uri!("file:///dir/one.rs")
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 7)
            );
            assert_eq!(params.new_name, "THREE");
            Ok(Some(lsp::WorkspaceEdit {
                changes: Some(
                    [
                        (
                            lsp::Url::from_file_path(path!("/dir/one.rs")).unwrap(),
                            vec![lsp::TextEdit::new(
                                lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                                "THREE".to_string(),
                            )],
                        ),
                        (
                            lsp::Url::from_file_path(path!("/dir/two.rs")).unwrap(),
                            vec![
                                lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 24),
                                        lsp::Position::new(0, 27),
                                    ),
                                    "THREE".to_string(),
                                ),
                                lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 35),
                                        lsp::Position::new(0, 38),
                                    ),
                                    "THREE".to_string(),
                                ),
                            ],
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            }))
        })
        .next()
        .await
        .unwrap();
    let mut transaction = response.await.unwrap().0;
    assert_eq!(transaction.len(), 2);
    assert_eq!(
        transaction
            .remove_entry(&buffer)
            .unwrap()
            .0
            .update(cx, |buffer, _| buffer.text()),
        "const THREE: usize = 1;"
    );
    assert_eq!(
        transaction
            .into_keys()
            .next()
            .unwrap()
            .update(cx, |buffer, _| buffer.text()),
        "const TWO: usize = one::THREE + one::THREE;"
    );
}

#[gpui::test]
async fn test_search(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "one.rs": "const ONE: usize = 1;",
            "two.rs": "const TWO: usize = one::ONE + one::ONE;",
            "three.rs": "const THREE: usize = one::ONE + two::TWO;",
            "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                "TWO",
                false,
                true,
                false,
                Default::default(),
                Default::default(),
                false,
                None
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/two.rs").to_string(), vec![6..9]),
            (path!("dir/three.rs").to_string(), vec![37..40])
        ])
    );

    let buffer_4 = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/dir/four.rs"), cx)
        })
        .await
        .unwrap();
    buffer_4.update(cx, |buffer, cx| {
        let text = "two::TWO";
        buffer.edit([(20..28, text), (31..43, text)], None, cx);
    });

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                "TWO",
                false,
                true,
                false,
                Default::default(),
                Default::default(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/two.rs").to_string(), vec![6..9]),
            (path!("dir/three.rs").to_string(), vec![37..40]),
            (path!("dir/four.rs").to_string(), vec![25..28, 36..39])
        ])
    );
}

#[gpui::test]
async fn test_search_with_inclusions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let search_query = "file";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "one.rs": r#"// Rust file one"#,
            "one.ts": r#"// TypeScript file one"#,
            "two.rs": r#"// Rust file two"#,
            "two.ts": r#"// TypeScript file two"#,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                PathMatcher::new(&["*.odd".to_owned()]).unwrap(),
                Default::default(),
                false,
                None
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "If no inclusions match, no files should be returned"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                PathMatcher::new(&["*.rs".to_owned()]).unwrap(),
                Default::default(),
                false,
                None
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.rs").to_string(), vec![8..12]),
            (path!("dir/two.rs").to_string(), vec![8..12]),
        ]),
        "Rust only search should give only Rust files"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                PathMatcher::new(&["*.ts".to_owned(), "*.odd".to_owned()]).unwrap(),
                Default::default(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.ts").to_string(), vec![14..18]),
            (path!("dir/two.ts").to_string(), vec![14..18]),
        ]),
        "TypeScript only search should give only TypeScript files, even if other inclusions don't match anything"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                PathMatcher::new(&["*.rs".to_owned(), "*.ts".to_owned(), "*.odd".to_owned()])
                    .unwrap(),
                Default::default(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/two.ts").to_string(), vec![14..18]),
            (path!("dir/one.rs").to_string(), vec![8..12]),
            (path!("dir/one.ts").to_string(), vec![14..18]),
            (path!("dir/two.rs").to_string(), vec![8..12]),
        ]),
        "Rust and typescript search should give both Rust and TypeScript files, even if other inclusions don't match anything"
    );
}

#[gpui::test]
async fn test_search_with_exclusions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let search_query = "file";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "one.rs": r#"// Rust file one"#,
            "one.ts": r#"// TypeScript file one"#,
            "two.rs": r#"// Rust file two"#,
            "two.ts": r#"// TypeScript file two"#,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Default::default(),
                PathMatcher::new(&["*.odd".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.rs").to_string(), vec![8..12]),
            (path!("dir/one.ts").to_string(), vec![14..18]),
            (path!("dir/two.rs").to_string(), vec![8..12]),
            (path!("dir/two.ts").to_string(), vec![14..18]),
        ]),
        "If no exclusions match, all files should be returned"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Default::default(),
                PathMatcher::new(&["*.rs".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.ts").to_string(), vec![14..18]),
            (path!("dir/two.ts").to_string(), vec![14..18]),
        ]),
        "Rust exclusion search should give only TypeScript files"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Default::default(),
                PathMatcher::new(&["*.ts".to_owned(), "*.odd".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.rs").to_string(), vec![8..12]),
            (path!("dir/two.rs").to_string(), vec![8..12]),
        ]),
        "TypeScript exclusion search should give only Rust files, even if other exclusions don't match anything"
    );

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Default::default(),
                PathMatcher::new(&["*.rs".to_owned(), "*.ts".to_owned(), "*.odd".to_owned()])
                    .unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "Rust and typescript exclusion should give no files, even if other exclusions don't match anything"
    );
}

#[gpui::test]
async fn test_search_with_buffer_exclusions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let search_query = "file";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "one.rs": r#"// Rust file one"#,
            "one.ts": r#"// TypeScript file one"#,
            "two.rs": r#"// Rust file two"#,
            "two.ts": r#"// TypeScript file two"#,
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let _buffer = project.update(cx, |project, cx| {
        let buffer = project.create_local_buffer("file", None, cx);
        project.mark_buffer_as_non_searchable(buffer.read(cx).remote_id(), cx);
        buffer
    });

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Default::default(),
                PathMatcher::new(&["*.odd".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.rs").to_string(), vec![8..12]),
            (path!("dir/one.ts").to_string(), vec![14..18]),
            (path!("dir/two.rs").to_string(), vec![8..12]),
            (path!("dir/two.ts").to_string(), vec![14..18]),
        ]),
        "If no exclusions match, all files should be returned"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Default::default(),
                PathMatcher::new(&["*.rs".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.ts").to_string(), vec![14..18]),
            (path!("dir/two.ts").to_string(), vec![14..18]),
        ]),
        "Rust exclusion search should give only TypeScript files"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Default::default(),
                PathMatcher::new(&["*.ts".to_owned(), "*.odd".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.rs").to_string(), vec![8..12]),
            (path!("dir/two.rs").to_string(), vec![8..12]),
        ]),
        "TypeScript exclusion search should give only Rust files, even if other exclusions don't match anything"
    );

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Default::default(),
                PathMatcher::new(&["*.rs".to_owned(), "*.ts".to_owned(), "*.odd".to_owned()])
                    .unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "Rust and typescript exclusion should give no files, even if other exclusions don't match anything"
    );
}

#[gpui::test]
async fn test_search_with_exclusions_and_inclusions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let search_query = "file";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "one.rs": r#"// Rust file one"#,
            "one.ts": r#"// TypeScript file one"#,
            "two.rs": r#"// Rust file two"#,
            "two.ts": r#"// TypeScript file two"#,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                PathMatcher::new(&["*.odd".to_owned()]).unwrap(),
                PathMatcher::new(&["*.odd".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "If both no exclusions and inclusions match, exclusions should win and return nothing"
    );

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                PathMatcher::new(&["*.ts".to_owned()]).unwrap(),
                PathMatcher::new(&["*.ts".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "If both TypeScript exclusions and inclusions match, exclusions should win and return nothing files."
    );

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                PathMatcher::new(&["*.ts".to_owned(), "*.odd".to_owned()]).unwrap(),
                PathMatcher::new(&["*.ts".to_owned(), "*.odd".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "Non-matching inclusions and exclusions should not change that."
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                PathMatcher::new(&["*.ts".to_owned(), "*.odd".to_owned()]).unwrap(),
                PathMatcher::new(&["*.rs".to_owned(), "*.odd".to_owned()]).unwrap(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.ts").to_string(), vec![14..18]),
            (path!("dir/two.ts").to_string(), vec![14..18]),
        ]),
        "Non-intersecting TypeScript inclusions and Rust exclusions should return TypeScript files"
    );
}

#[gpui::test]
async fn test_search_multiple_worktrees_with_inclusions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/worktree-a"),
        json!({
            "haystack.rs": r#"// NEEDLE"#,
            "haystack.ts": r#"// NEEDLE"#,
        }),
    )
    .await;
    fs.insert_tree(
        path!("/worktree-b"),
        json!({
            "haystack.rs": r#"// NEEDLE"#,
            "haystack.ts": r#"// NEEDLE"#,
        }),
    )
    .await;

    let project = Project::test(
        fs.clone(),
        [path!("/worktree-a").as_ref(), path!("/worktree-b").as_ref()],
        cx,
    )
    .await;

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                "NEEDLE",
                false,
                true,
                false,
                PathMatcher::new(&["worktree-a/*.rs".to_owned()]).unwrap(),
                Default::default(),
                true,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([(path!("worktree-a/haystack.rs").to_string(), vec![3..9])]),
        "should only return results from included worktree"
    );
    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                "NEEDLE",
                false,
                true,
                false,
                PathMatcher::new(&["worktree-b/*.rs".to_owned()]).unwrap(),
                Default::default(),
                true,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([(path!("worktree-b/haystack.rs").to_string(), vec![3..9])]),
        "should only return results from included worktree"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                "NEEDLE",
                false,
                true,
                false,
                PathMatcher::new(&["*.ts".to_owned()]).unwrap(),
                Default::default(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("worktree-a/haystack.ts").to_string(), vec![3..9]),
            (path!("worktree-b/haystack.ts").to_string(), vec![3..9])
        ]),
        "should return results from both worktrees"
    );
}

#[gpui::test]
async fn test_search_in_gitignored_dirs(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        path!("/dir"),
        json!({
            ".git": {},
            ".gitignore": "**/target\n/node_modules\n",
            "target": {
                "index.txt": "index_key:index_value"
            },
            "node_modules": {
                "eslint": {
                    "index.ts": "const eslint_key = 'eslint value'",
                    "package.json": r#"{ "some_key": "some value" }"#,
                },
                "prettier": {
                    "index.ts": "const prettier_key = 'prettier value'",
                    "package.json": r#"{ "other_key": "other value" }"#,
                },
            },
            "package.json": r#"{ "main_key": "main value" }"#,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

    let query = "key";
    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                query,
                false,
                false,
                false,
                Default::default(),
                Default::default(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([(path!("dir/package.json").to_string(), vec![8..11])]),
        "Only one non-ignored file should have the query"
    );

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                query,
                false,
                false,
                true,
                Default::default(),
                Default::default(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            (path!("dir/package.json").to_string(), vec![8..11]),
            (path!("dir/target/index.txt").to_string(), vec![6..9]),
            (
                path!("dir/node_modules/prettier/package.json").to_string(),
                vec![9..12]
            ),
            (
                path!("dir/node_modules/prettier/index.ts").to_string(),
                vec![15..18]
            ),
            (
                path!("dir/node_modules/eslint/index.ts").to_string(),
                vec![13..16]
            ),
            (
                path!("dir/node_modules/eslint/package.json").to_string(),
                vec![8..11]
            ),
        ]),
        "Unrestricted search with ignored directories should find every file with the query"
    );

    let files_to_include = PathMatcher::new(&["node_modules/prettier/**".to_owned()]).unwrap();
    let files_to_exclude = PathMatcher::new(&["*.ts".to_owned()]).unwrap();
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                query,
                false,
                false,
                true,
                files_to_include,
                files_to_exclude,
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([(
            path!("dir/node_modules/prettier/package.json").to_string(),
            vec![9..12]
        )]),
        "With search including ignored prettier directory and excluding TS files, only one file should be found"
    );
}

#[gpui::test]
async fn test_search_with_unicode(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "one.rs": "// ПРИВЕТ? привет!",
            "two.rs": "// ПРИВЕТ.",
            "three.rs": "// привет",
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

    let unicode_case_sensitive_query = SearchQuery::text(
        "привет",
        false,
        true,
        false,
        Default::default(),
        Default::default(),
        false,
        None,
    );
    assert_matches!(unicode_case_sensitive_query, Ok(SearchQuery::Text { .. }));
    assert_eq!(
        search(&project, unicode_case_sensitive_query.unwrap(), cx)
            .await
            .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.rs").to_string(), vec![17..29]),
            (path!("dir/three.rs").to_string(), vec![3..15]),
        ])
    );

    let unicode_case_insensitive_query = SearchQuery::text(
        "привет",
        false,
        false,
        false,
        Default::default(),
        Default::default(),
        false,
        None,
    );
    assert_matches!(
        unicode_case_insensitive_query,
        Ok(SearchQuery::Regex { .. })
    );
    assert_eq!(
        search(&project, unicode_case_insensitive_query.unwrap(), cx)
            .await
            .unwrap(),
        HashMap::from_iter([
            (path!("dir/one.rs").to_string(), vec![3..15, 17..29]),
            (path!("dir/two.rs").to_string(), vec![3..15]),
            (path!("dir/three.rs").to_string(), vec![3..15]),
        ])
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                "привет.",
                false,
                false,
                false,
                Default::default(),
                Default::default(),
                false,
                None,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([(path!("dir/two.rs").to_string(), vec![3..16]),])
    );
}

#[gpui::test]
async fn test_create_entry(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
    fs.insert_tree(
        "/one/two",
        json!({
            "three": {
                "a.txt": "",
                "four": {}
            },
            "c.rs": ""
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/one/two/three".as_ref()], cx).await;
    project
        .update(cx, |project, cx| {
            let id = project.worktrees(cx).next().unwrap().read(cx).id();
            project.create_entry((id, "b.."), true, cx)
        })
        .await
        .unwrap()
        .to_included()
        .unwrap();

    // Can't create paths outside the project
    let result = project
        .update(cx, |project, cx| {
            let id = project.worktrees(cx).next().unwrap().read(cx).id();
            project.create_entry((id, "../../boop"), true, cx)
        })
        .await;
    assert!(result.is_err());

    // Can't create paths with '..'
    let result = project
        .update(cx, |project, cx| {
            let id = project.worktrees(cx).next().unwrap().read(cx).id();
            project.create_entry((id, "four/../beep"), true, cx)
        })
        .await;
    assert!(result.is_err());

    assert_eq!(
        fs.paths(true),
        vec![
            PathBuf::from(path!("/")),
            PathBuf::from(path!("/one")),
            PathBuf::from(path!("/one/two")),
            PathBuf::from(path!("/one/two/c.rs")),
            PathBuf::from(path!("/one/two/three")),
            PathBuf::from(path!("/one/two/three/a.txt")),
            PathBuf::from(path!("/one/two/three/b..")),
            PathBuf::from(path!("/one/two/three/four")),
        ]
    );

    // And we cannot open buffers with '..'
    let result = project
        .update(cx, |project, cx| {
            let id = project.worktrees(cx).next().unwrap().read(cx).id();
            project.open_buffer((id, "../c.rs"), cx)
        })
        .await;
    assert!(result.is_err())
}

#[gpui::test]
async fn test_multiple_language_server_hovers(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.tsx": "a",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(tsx_lang());
    let language_server_names = [
        "TypeScriptServer",
        "TailwindServer",
        "ESLintServer",
        "NoHoverCapabilitiesServer",
    ];
    let mut language_servers = [
        language_registry.register_fake_lsp(
            "tsx",
            FakeLspAdapter {
                name: language_server_names[0],
                capabilities: lsp::ServerCapabilities {
                    hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        ),
        language_registry.register_fake_lsp(
            "tsx",
            FakeLspAdapter {
                name: language_server_names[1],
                capabilities: lsp::ServerCapabilities {
                    hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        ),
        language_registry.register_fake_lsp(
            "tsx",
            FakeLspAdapter {
                name: language_server_names[2],
                capabilities: lsp::ServerCapabilities {
                    hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        ),
        language_registry.register_fake_lsp(
            "tsx",
            FakeLspAdapter {
                name: language_server_names[3],
                capabilities: lsp::ServerCapabilities {
                    hover_provider: None,
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        ),
    ];

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.tsx"), cx)
        })
        .await
        .unwrap();
    cx.executor().run_until_parked();

    let mut servers_with_hover_requests = HashMap::default();
    for i in 0..language_server_names.len() {
        let new_server = language_servers[i].next().await.unwrap_or_else(|| {
            panic!(
                "Failed to get language server #{i} with name {}",
                &language_server_names[i]
            )
        });
        let new_server_name = new_server.server.name();
        assert!(
            !servers_with_hover_requests.contains_key(&new_server_name),
            "Unexpected: initialized server with the same name twice. Name: `{new_server_name}`"
        );
        match new_server_name.as_ref() {
            "TailwindServer" | "TypeScriptServer" => {
                servers_with_hover_requests.insert(
                    new_server_name.clone(),
                    new_server.set_request_handler::<lsp::request::HoverRequest, _, _>(
                        move |_, _| {
                            let name = new_server_name.clone();
                            async move {
                                Ok(Some(lsp::Hover {
                                    contents: lsp::HoverContents::Scalar(
                                        lsp::MarkedString::String(format!("{name} hover")),
                                    ),
                                    range: None,
                                }))
                            }
                        },
                    ),
                );
            }
            "ESLintServer" => {
                servers_with_hover_requests.insert(
                    new_server_name,
                    new_server.set_request_handler::<lsp::request::HoverRequest, _, _>(
                        |_, _| async move { Ok(None) },
                    ),
                );
            }
            "NoHoverCapabilitiesServer" => {
                let _never_handled = new_server
                    .set_request_handler::<lsp::request::HoverRequest, _, _>(|_, _| async move {
                        panic!(
                            "Should not call for hovers server with no corresponding capabilities"
                        )
                    });
            }
            unexpected => panic!("Unexpected server name: {unexpected}"),
        }
    }

    let hover_task = project.update(cx, |project, cx| {
        project.hover(&buffer, Point::new(0, 0), cx)
    });
    let _: Vec<()> = futures::future::join_all(servers_with_hover_requests.into_values().map(
        |mut hover_request| async move {
            hover_request
                .next()
                .await
                .expect("All hover requests should have been triggered")
        },
    ))
    .await;
    assert_eq!(
        vec!["TailwindServer hover", "TypeScriptServer hover"],
        hover_task
            .await
            .into_iter()
            .map(|hover| hover.contents.iter().map(|block| &block.text).join("|"))
            .sorted()
            .collect::<Vec<_>>(),
        "Should receive hover responses from all related servers with hover capabilities"
    );
}

#[gpui::test]
async fn test_hovers_with_empty_parts(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.ts": "a",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.ts"), cx)
        })
        .await
        .unwrap();
    cx.executor().run_until_parked();

    let fake_server = fake_language_servers
        .next()
        .await
        .expect("failed to get the language server");

    let mut request_handled = fake_server.set_request_handler::<lsp::request::HoverRequest, _, _>(
        move |_, _| async move {
            Ok(Some(lsp::Hover {
                contents: lsp::HoverContents::Array(vec![
                    lsp::MarkedString::String("".to_string()),
                    lsp::MarkedString::String("      ".to_string()),
                    lsp::MarkedString::String("\n\n\n".to_string()),
                ]),
                range: None,
            }))
        },
    );

    let hover_task = project.update(cx, |project, cx| {
        project.hover(&buffer, Point::new(0, 0), cx)
    });
    let () = request_handled
        .next()
        .await
        .expect("All hover requests should have been triggered");
    assert_eq!(
        Vec::<String>::new(),
        hover_task
            .await
            .into_iter()
            .map(|hover| hover.contents.iter().map(|block| &block.text).join("|"))
            .sorted()
            .collect::<Vec<_>>(),
        "Empty hover parts should be ignored"
    );
}

#[gpui::test]
async fn test_code_actions_only_kinds(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.ts": "a",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.ts"), cx)
        })
        .await
        .unwrap();
    cx.executor().run_until_parked();

    let fake_server = fake_language_servers
        .next()
        .await
        .expect("failed to get the language server");

    let mut request_handled = fake_server
        .set_request_handler::<lsp::request::CodeActionRequest, _, _>(move |_, _| async move {
            Ok(Some(vec![
                lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                    title: "organize imports".to_string(),
                    kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
                    ..lsp::CodeAction::default()
                }),
                lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                    title: "fix code".to_string(),
                    kind: Some(CodeActionKind::SOURCE_FIX_ALL),
                    ..lsp::CodeAction::default()
                }),
            ]))
        });

    let code_actions_task = project.update(cx, |project, cx| {
        project.code_actions(
            &buffer,
            0..buffer.read(cx).len(),
            Some(vec![CodeActionKind::SOURCE_ORGANIZE_IMPORTS]),
            cx,
        )
    });

    let () = request_handled
        .next()
        .await
        .expect("The code action request should have been triggered");

    let code_actions = code_actions_task.await.unwrap();
    assert_eq!(code_actions.len(), 1);
    assert_eq!(
        code_actions[0].lsp_action.action_kind(),
        Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS)
    );
}

#[gpui::test]
async fn test_multiple_language_server_actions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.tsx": "a",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(tsx_lang());
    let language_server_names = [
        "TypeScriptServer",
        "TailwindServer",
        "ESLintServer",
        "NoActionsCapabilitiesServer",
    ];

    let mut language_server_rxs = [
        language_registry.register_fake_lsp(
            "tsx",
            FakeLspAdapter {
                name: language_server_names[0],
                capabilities: lsp::ServerCapabilities {
                    code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        ),
        language_registry.register_fake_lsp(
            "tsx",
            FakeLspAdapter {
                name: language_server_names[1],
                capabilities: lsp::ServerCapabilities {
                    code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        ),
        language_registry.register_fake_lsp(
            "tsx",
            FakeLspAdapter {
                name: language_server_names[2],
                capabilities: lsp::ServerCapabilities {
                    code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        ),
        language_registry.register_fake_lsp(
            "tsx",
            FakeLspAdapter {
                name: language_server_names[3],
                capabilities: lsp::ServerCapabilities {
                    code_action_provider: None,
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        ),
    ];

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.tsx"), cx)
        })
        .await
        .unwrap();
    cx.executor().run_until_parked();

    let mut servers_with_actions_requests = HashMap::default();
    for i in 0..language_server_names.len() {
        let new_server = language_server_rxs[i].next().await.unwrap_or_else(|| {
            panic!(
                "Failed to get language server #{i} with name {}",
                &language_server_names[i]
            )
        });
        let new_server_name = new_server.server.name();

        assert!(
            !servers_with_actions_requests.contains_key(&new_server_name),
            "Unexpected: initialized server with the same name twice. Name: `{new_server_name}`"
        );
        match new_server_name.0.as_ref() {
            "TailwindServer" | "TypeScriptServer" => {
                servers_with_actions_requests.insert(
                    new_server_name.clone(),
                    new_server.set_request_handler::<lsp::request::CodeActionRequest, _, _>(
                        move |_, _| {
                            let name = new_server_name.clone();
                            async move {
                                Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                                    lsp::CodeAction {
                                        title: format!("{name} code action"),
                                        ..lsp::CodeAction::default()
                                    },
                                )]))
                            }
                        },
                    ),
                );
            }
            "ESLintServer" => {
                servers_with_actions_requests.insert(
                    new_server_name,
                    new_server.set_request_handler::<lsp::request::CodeActionRequest, _, _>(
                        |_, _| async move { Ok(None) },
                    ),
                );
            }
            "NoActionsCapabilitiesServer" => {
                let _never_handled = new_server
                    .set_request_handler::<lsp::request::CodeActionRequest, _, _>(|_, _| async move {
                        panic!(
                            "Should not call for code actions server with no corresponding capabilities"
                        )
                    });
            }
            unexpected => panic!("Unexpected server name: {unexpected}"),
        }
    }

    let code_actions_task = project.update(cx, |project, cx| {
        project.code_actions(&buffer, 0..buffer.read(cx).len(), None, cx)
    });

    // cx.run_until_parked();
    let _: Vec<()> = futures::future::join_all(servers_with_actions_requests.into_values().map(
        |mut code_actions_request| async move {
            code_actions_request
                .next()
                .await
                .expect("All code actions requests should have been triggered")
        },
    ))
    .await;
    assert_eq!(
        vec!["TailwindServer code action", "TypeScriptServer code action"],
        code_actions_task
            .await
            .unwrap()
            .into_iter()
            .map(|code_action| code_action.lsp_action.title().to_owned())
            .sorted()
            .collect::<Vec<_>>(),
        "Should receive code actions responses from all related servers with hover capabilities"
    );
}

#[gpui::test]
async fn test_reordering_worktrees(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": "let a = 1;",
            "b.rs": "let b = 2;",
            "c.rs": "let c = 2;",
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [
            "/dir/a.rs".as_ref(),
            "/dir/b.rs".as_ref(),
            "/dir/c.rs".as_ref(),
        ],
        cx,
    )
    .await;

    // check the initial state and get the worktrees
    let (worktree_a, worktree_b, worktree_c) = project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let worktree_a = worktrees[0].read(cx);
        let worktree_b = worktrees[1].read(cx);
        let worktree_c = worktrees[2].read(cx);

        // check they start in the right order
        assert_eq!(worktree_a.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(worktree_b.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(worktree_c.abs_path().to_str().unwrap(), "/dir/c.rs");

        (
            worktrees[0].clone(),
            worktrees[1].clone(),
            worktrees[2].clone(),
        )
    });

    // move first worktree to after the second
    // [a, b, c] -> [b, a, c]
    project
        .update(cx, |project, cx| {
            let first = worktree_a.read(cx);
            let second = worktree_b.read(cx);
            project.move_worktree(first.id(), second.id(), cx)
        })
        .expect("moving first after second");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/c.rs");
    });

    // move the second worktree to before the first
    // [b, a, c] -> [a, b, c]
    project
        .update(cx, |project, cx| {
            let second = worktree_a.read(cx);
            let first = worktree_b.read(cx);
            project.move_worktree(first.id(), second.id(), cx)
        })
        .expect("moving second before first");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/c.rs");
    });

    // move the second worktree to after the third
    // [a, b, c] -> [a, c, b]
    project
        .update(cx, |project, cx| {
            let second = worktree_b.read(cx);
            let third = worktree_c.read(cx);
            project.move_worktree(second.id(), third.id(), cx)
        })
        .expect("moving second after third");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/c.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/b.rs");
    });

    // move the third worktree to before the second
    // [a, c, b] -> [a, b, c]
    project
        .update(cx, |project, cx| {
            let third = worktree_c.read(cx);
            let second = worktree_b.read(cx);
            project.move_worktree(third.id(), second.id(), cx)
        })
        .expect("moving third before second");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/c.rs");
    });

    // move the first worktree to after the third
    // [a, b, c] -> [b, c, a]
    project
        .update(cx, |project, cx| {
            let first = worktree_a.read(cx);
            let third = worktree_c.read(cx);
            project.move_worktree(first.id(), third.id(), cx)
        })
        .expect("moving first after third");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/c.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/a.rs");
    });

    // move the third worktree to before the first
    // [b, c, a] -> [a, b, c]
    project
        .update(cx, |project, cx| {
            let third = worktree_a.read(cx);
            let first = worktree_b.read(cx);
            project.move_worktree(third.id(), first.id(), cx)
        })
        .expect("moving third before first");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/c.rs");
    });
}

#[gpui::test]
async fn test_unstaged_diff_for_buffer(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let staged_contents = r#"
        fn main() {
            println!("hello world");
        }
    "#
    .unindent();
    let file_contents = r#"
        // print goodbye
        fn main() {
            println!("goodbye world");
        }
    "#
    .unindent();

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/dir",
        json!({
            ".git": {},
           "src": {
               "main.rs": file_contents,
           }
        }),
    )
    .await;

    fs.set_index_for_repo(
        Path::new("/dir/.git"),
        &[("src/main.rs".into(), staged_contents)],
    );

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/src/main.rs", cx)
        })
        .await
        .unwrap();
    let unstaged_diff = project
        .update(cx, |project, cx| {
            project.open_unstaged_diff(buffer.clone(), cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();
    unstaged_diff.update(cx, |unstaged_diff, cx| {
        let snapshot = buffer.read(cx).snapshot();
        assert_hunks(
            unstaged_diff.hunks(&snapshot, cx),
            &snapshot,
            &unstaged_diff.base_text_string().unwrap(),
            &[
                (0..1, "", "// print goodbye\n", DiffHunkStatus::added_none()),
                (
                    2..3,
                    "    println!(\"hello world\");\n",
                    "    println!(\"goodbye world\");\n",
                    DiffHunkStatus::modified_none(),
                ),
            ],
        );
    });

    let staged_contents = r#"
        // print goodbye
        fn main() {
        }
    "#
    .unindent();

    fs.set_index_for_repo(
        Path::new("/dir/.git"),
        &[("src/main.rs".into(), staged_contents)],
    );

    cx.run_until_parked();
    unstaged_diff.update(cx, |unstaged_diff, cx| {
        let snapshot = buffer.read(cx).snapshot();
        assert_hunks(
            unstaged_diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot, cx),
            &snapshot,
            &unstaged_diff.base_text().text(),
            &[(
                2..3,
                "",
                "    println!(\"goodbye world\");\n",
                DiffHunkStatus::added_none(),
            )],
        );
    });
}

#[gpui::test]
async fn test_uncommitted_diff_for_buffer(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let committed_contents = r#"
        fn main() {
            println!("hello world");
        }
    "#
    .unindent();
    let staged_contents = r#"
        fn main() {
            println!("goodbye world");
        }
    "#
    .unindent();
    let file_contents = r#"
        // print goodbye
        fn main() {
            println!("goodbye world");
        }
    "#
    .unindent();

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/dir",
        json!({
            ".git": {},
           "src": {
               "modification.rs": file_contents,
           }
        }),
    )
    .await;

    fs.set_head_for_repo(
        Path::new("/dir/.git"),
        &[
            ("src/modification.rs".into(), committed_contents),
            ("src/deletion.rs".into(), "// the-deleted-contents\n".into()),
        ],
        "deadbeef",
    );
    fs.set_index_for_repo(
        Path::new("/dir/.git"),
        &[
            ("src/modification.rs".into(), staged_contents),
            ("src/deletion.rs".into(), "// the-deleted-contents\n".into()),
        ],
    );

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    let language = rust_lang();
    language_registry.add(language.clone());

    let buffer_1 = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/src/modification.rs", cx)
        })
        .await
        .unwrap();
    let diff_1 = project
        .update(cx, |project, cx| {
            project.open_uncommitted_diff(buffer_1.clone(), cx)
        })
        .await
        .unwrap();
    diff_1.read_with(cx, |diff, _| {
        assert_eq!(diff.base_text().language().cloned(), Some(language))
    });
    cx.run_until_parked();
    diff_1.update(cx, |diff, cx| {
        let snapshot = buffer_1.read(cx).snapshot();
        assert_hunks(
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..1,
                    "",
                    "// print goodbye\n",
                    DiffHunkStatus::added(DiffHunkSecondaryStatus::HasSecondaryHunk),
                ),
                (
                    2..3,
                    "    println!(\"hello world\");\n",
                    "    println!(\"goodbye world\");\n",
                    DiffHunkStatus::modified_none(),
                ),
            ],
        );
    });

    // Reset HEAD to a version that differs from both the buffer and the index.
    let committed_contents = r#"
        // print goodbye
        fn main() {
        }
    "#
    .unindent();
    fs.set_head_for_repo(
        Path::new("/dir/.git"),
        &[
            ("src/modification.rs".into(), committed_contents.clone()),
            ("src/deletion.rs".into(), "// the-deleted-contents\n".into()),
        ],
        "deadbeef",
    );

    // Buffer now has an unstaged hunk.
    cx.run_until_parked();
    diff_1.update(cx, |diff, cx| {
        let snapshot = buffer_1.read(cx).snapshot();
        assert_hunks(
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot, cx),
            &snapshot,
            &diff.base_text().text(),
            &[(
                2..3,
                "",
                "    println!(\"goodbye world\");\n",
                DiffHunkStatus::added_none(),
            )],
        );
    });

    // Open a buffer for a file that's been deleted.
    let buffer_2 = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/src/deletion.rs", cx)
        })
        .await
        .unwrap();
    let diff_2 = project
        .update(cx, |project, cx| {
            project.open_uncommitted_diff(buffer_2.clone(), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();
    diff_2.update(cx, |diff, cx| {
        let snapshot = buffer_2.read(cx).snapshot();
        assert_hunks(
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[(
                0..0,
                "// the-deleted-contents\n",
                "",
                DiffHunkStatus::deleted(DiffHunkSecondaryStatus::HasSecondaryHunk),
            )],
        );
    });

    // Stage the deletion of this file
    fs.set_index_for_repo(
        Path::new("/dir/.git"),
        &[("src/modification.rs".into(), committed_contents.clone())],
    );
    cx.run_until_parked();
    diff_2.update(cx, |diff, cx| {
        let snapshot = buffer_2.read(cx).snapshot();
        assert_hunks(
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[(
                0..0,
                "// the-deleted-contents\n",
                "",
                DiffHunkStatus::deleted(DiffHunkSecondaryStatus::NoSecondaryHunk),
            )],
        );
    });
}

#[gpui::test]
async fn test_staging_hunks(cx: &mut gpui::TestAppContext) {
    use DiffHunkSecondaryStatus::*;
    init_test(cx);

    let committed_contents = r#"
        zero
        one
        two
        three
        four
        five
    "#
    .unindent();
    let file_contents = r#"
        one
        TWO
        three
        FOUR
        five
    "#
    .unindent();

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/dir",
        json!({
            ".git": {},
            "file.txt": file_contents.clone()
        }),
    )
    .await;

    fs.set_head_and_index_for_repo(
        "/dir/.git".as_ref(),
        &[("file.txt".into(), committed_contents.clone())],
    );

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/file.txt", cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let uncommitted_diff = project
        .update(cx, |project, cx| {
            project.open_uncommitted_diff(buffer.clone(), cx)
        })
        .await
        .unwrap();
    let mut diff_events = cx.events(&uncommitted_diff);

    // The hunks are initially unstaged.
    uncommitted_diff.read_with(cx, |diff, cx| {
        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..0,
                    "zero\n",
                    "",
                    DiffHunkStatus::deleted(HasSecondaryHunk),
                ),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
            ],
        );
    });

    // Stage a hunk. It appears as optimistically staged.
    uncommitted_diff.update(cx, |diff, cx| {
        let range =
            snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_before(Point::new(2, 0));
        let hunks = diff
            .hunks_intersecting_range(range, &snapshot, cx)
            .collect::<Vec<_>>();
        diff.stage_or_unstage_hunks(true, &hunks, &snapshot, true, cx);

        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..0,
                    "zero\n",
                    "",
                    DiffHunkStatus::deleted(HasSecondaryHunk),
                ),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(SecondaryHunkRemovalPending),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
            ],
        );
    });

    // The diff emits a change event for the range of the staged hunk.
    assert!(matches!(
        diff_events.next().await.unwrap(),
        BufferDiffEvent::HunksStagedOrUnstaged(_)
    ));
    let event = diff_events.next().await.unwrap();
    if let BufferDiffEvent::DiffChanged {
        changed_range: Some(changed_range),
    } = event
    {
        let changed_range = changed_range.to_point(&snapshot);
        assert_eq!(changed_range, Point::new(1, 0)..Point::new(2, 0));
    } else {
        panic!("Unexpected event {event:?}");
    }

    // When the write to the index completes, it appears as staged.
    cx.run_until_parked();
    uncommitted_diff.update(cx, |diff, cx| {
        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..0,
                    "zero\n",
                    "",
                    DiffHunkStatus::deleted(HasSecondaryHunk),
                ),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(NoSecondaryHunk),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
            ],
        );
    });

    // The diff emits a change event for the changed index text.
    let event = diff_events.next().await.unwrap();
    if let BufferDiffEvent::DiffChanged {
        changed_range: Some(changed_range),
    } = event
    {
        let changed_range = changed_range.to_point(&snapshot);
        assert_eq!(changed_range, Point::new(0, 0)..Point::new(4, 0));
    } else {
        panic!("Unexpected event {event:?}");
    }

    // Simulate a problem writing to the git index.
    fs.set_error_message_for_index_write(
        "/dir/.git".as_ref(),
        Some("failed to write git index".into()),
    );

    // Stage another hunk.
    uncommitted_diff.update(cx, |diff, cx| {
        let range =
            snapshot.anchor_before(Point::new(3, 0))..snapshot.anchor_before(Point::new(4, 0));
        let hunks = diff
            .hunks_intersecting_range(range, &snapshot, cx)
            .collect::<Vec<_>>();
        diff.stage_or_unstage_hunks(true, &hunks, &snapshot, true, cx);

        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..0,
                    "zero\n",
                    "",
                    DiffHunkStatus::deleted(HasSecondaryHunk),
                ),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(NoSecondaryHunk),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(SecondaryHunkRemovalPending),
                ),
            ],
        );
    });
    assert!(matches!(
        diff_events.next().await.unwrap(),
        BufferDiffEvent::HunksStagedOrUnstaged(_)
    ));
    let event = diff_events.next().await.unwrap();
    if let BufferDiffEvent::DiffChanged {
        changed_range: Some(changed_range),
    } = event
    {
        let changed_range = changed_range.to_point(&snapshot);
        assert_eq!(changed_range, Point::new(3, 0)..Point::new(4, 0));
    } else {
        panic!("Unexpected event {event:?}");
    }

    // When the write fails, the hunk returns to being unstaged.
    cx.run_until_parked();
    uncommitted_diff.update(cx, |diff, cx| {
        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..0,
                    "zero\n",
                    "",
                    DiffHunkStatus::deleted(HasSecondaryHunk),
                ),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(NoSecondaryHunk),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
            ],
        );
    });

    let event = diff_events.next().await.unwrap();
    if let BufferDiffEvent::DiffChanged {
        changed_range: Some(changed_range),
    } = event
    {
        let changed_range = changed_range.to_point(&snapshot);
        assert_eq!(changed_range, Point::new(0, 0)..Point::new(5, 0));
    } else {
        panic!("Unexpected event {event:?}");
    }

    // Allow writing to the git index to succeed again.
    fs.set_error_message_for_index_write("/dir/.git".as_ref(), None);

    // Stage two hunks with separate operations.
    uncommitted_diff.update(cx, |diff, cx| {
        let hunks = diff.hunks(&snapshot, cx).collect::<Vec<_>>();
        diff.stage_or_unstage_hunks(true, &hunks[0..1], &snapshot, true, cx);
        diff.stage_or_unstage_hunks(true, &hunks[2..3], &snapshot, true, cx);
    });

    // Both staged hunks appear as pending.
    uncommitted_diff.update(cx, |diff, cx| {
        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..0,
                    "zero\n",
                    "",
                    DiffHunkStatus::deleted(SecondaryHunkRemovalPending),
                ),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(NoSecondaryHunk),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(SecondaryHunkRemovalPending),
                ),
            ],
        );
    });

    // Both staging operations take effect.
    cx.run_until_parked();
    uncommitted_diff.update(cx, |diff, cx| {
        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (0..0, "zero\n", "", DiffHunkStatus::deleted(NoSecondaryHunk)),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(NoSecondaryHunk),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(NoSecondaryHunk),
                ),
            ],
        );
    });
}

#[gpui::test(seeds(340, 472))]
async fn test_staging_hunks_with_delayed_fs_event(cx: &mut gpui::TestAppContext) {
    use DiffHunkSecondaryStatus::*;
    init_test(cx);

    let committed_contents = r#"
        zero
        one
        two
        three
        four
        five
    "#
    .unindent();
    let file_contents = r#"
        one
        TWO
        three
        FOUR
        five
    "#
    .unindent();

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/dir",
        json!({
            ".git": {},
            "file.txt": file_contents.clone()
        }),
    )
    .await;

    fs.set_head_for_repo(
        "/dir/.git".as_ref(),
        &[("file.txt".into(), committed_contents.clone())],
        "deadbeef",
    );
    fs.set_index_for_repo(
        "/dir/.git".as_ref(),
        &[("file.txt".into(), committed_contents.clone())],
    );

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/file.txt", cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let uncommitted_diff = project
        .update(cx, |project, cx| {
            project.open_uncommitted_diff(buffer.clone(), cx)
        })
        .await
        .unwrap();

    // The hunks are initially unstaged.
    uncommitted_diff.read_with(cx, |diff, cx| {
        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..0,
                    "zero\n",
                    "",
                    DiffHunkStatus::deleted(HasSecondaryHunk),
                ),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
            ],
        );
    });

    // Pause IO events
    fs.pause_events();

    // Stage the first hunk.
    uncommitted_diff.update(cx, |diff, cx| {
        let hunk = diff.hunks(&snapshot, cx).next().unwrap();
        diff.stage_or_unstage_hunks(true, &[hunk], &snapshot, true, cx);
        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..0,
                    "zero\n",
                    "",
                    DiffHunkStatus::deleted(SecondaryHunkRemovalPending),
                ),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
            ],
        );
    });

    // Stage the second hunk *before* receiving the FS event for the first hunk.
    cx.run_until_parked();
    uncommitted_diff.update(cx, |diff, cx| {
        let hunk = diff.hunks(&snapshot, cx).nth(1).unwrap();
        diff.stage_or_unstage_hunks(true, &[hunk], &snapshot, true, cx);
        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (
                    0..0,
                    "zero\n",
                    "",
                    DiffHunkStatus::deleted(SecondaryHunkRemovalPending),
                ),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(SecondaryHunkRemovalPending),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(HasSecondaryHunk),
                ),
            ],
        );
    });

    // Process the FS event for staging the first hunk (second event is still pending).
    fs.flush_events(1);
    cx.run_until_parked();

    // Stage the third hunk before receiving the second FS event.
    uncommitted_diff.update(cx, |diff, cx| {
        let hunk = diff.hunks(&snapshot, cx).nth(2).unwrap();
        diff.stage_or_unstage_hunks(true, &[hunk], &snapshot, true, cx);
    });

    // Wait for all remaining IO.
    cx.run_until_parked();
    fs.flush_events(fs.buffered_event_count());

    // Now all hunks are staged.
    cx.run_until_parked();
    uncommitted_diff.update(cx, |diff, cx| {
        assert_hunks(
            diff.hunks(&snapshot, cx),
            &snapshot,
            &diff.base_text_string().unwrap(),
            &[
                (0..0, "zero\n", "", DiffHunkStatus::deleted(NoSecondaryHunk)),
                (
                    1..2,
                    "two\n",
                    "TWO\n",
                    DiffHunkStatus::modified(NoSecondaryHunk),
                ),
                (
                    3..4,
                    "four\n",
                    "FOUR\n",
                    DiffHunkStatus::modified(NoSecondaryHunk),
                ),
            ],
        );
    });
}

#[gpui::test(iterations = 25)]
async fn test_staging_random_hunks(
    mut rng: StdRng,
    executor: BackgroundExecutor,
    cx: &mut gpui::TestAppContext,
) {
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(20);

    // Try to induce races between diff recalculation and index writes.
    if rng.gen_bool(0.5) {
        executor.deprioritize(*CALCULATE_DIFF_TASK);
    }

    use DiffHunkSecondaryStatus::*;
    init_test(cx);

    let committed_text = (0..30).map(|i| format!("line {i}\n")).collect::<String>();
    let index_text = committed_text.clone();
    let buffer_text = (0..30)
        .map(|i| match i % 5 {
            0 => format!("line {i} (modified)\n"),
            _ => format!("line {i}\n"),
        })
        .collect::<String>();

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        path!("/dir"),
        json!({
            ".git": {},
            "file.txt": buffer_text.clone()
        }),
    )
    .await;
    fs.set_head_for_repo(
        path!("/dir/.git").as_ref(),
        &[("file.txt".into(), committed_text.clone())],
        "deadbeef",
    );
    fs.set_index_for_repo(
        path!("/dir/.git").as_ref(),
        &[("file.txt".into(), index_text.clone())],
    );
    let repo = fs.open_repo(path!("/dir/.git").as_ref()).unwrap();

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/dir/file.txt"), cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let uncommitted_diff = project
        .update(cx, |project, cx| {
            project.open_uncommitted_diff(buffer.clone(), cx)
        })
        .await
        .unwrap();

    let mut hunks =
        uncommitted_diff.update(cx, |diff, cx| diff.hunks(&snapshot, cx).collect::<Vec<_>>());
    assert_eq!(hunks.len(), 6);

    for _i in 0..operations {
        let hunk_ix = rng.gen_range(0..hunks.len());
        let hunk = &mut hunks[hunk_ix];
        let row = hunk.range.start.row;

        if hunk.status().has_secondary_hunk() {
            log::info!("staging hunk at {row}");
            uncommitted_diff.update(cx, |diff, cx| {
                diff.stage_or_unstage_hunks(true, std::slice::from_ref(hunk), &snapshot, true, cx);
            });
            hunk.secondary_status = SecondaryHunkRemovalPending;
        } else {
            log::info!("unstaging hunk at {row}");
            uncommitted_diff.update(cx, |diff, cx| {
                diff.stage_or_unstage_hunks(false, std::slice::from_ref(hunk), &snapshot, true, cx);
            });
            hunk.secondary_status = SecondaryHunkAdditionPending;
        }

        for _ in 0..rng.gen_range(0..10) {
            log::info!("yielding");
            cx.executor().simulate_random_delay().await;
        }
    }

    cx.executor().run_until_parked();

    for hunk in &mut hunks {
        if hunk.secondary_status == SecondaryHunkRemovalPending {
            hunk.secondary_status = NoSecondaryHunk;
        } else if hunk.secondary_status == SecondaryHunkAdditionPending {
            hunk.secondary_status = HasSecondaryHunk;
        }
    }

    log::info!(
        "index text:\n{}",
        repo.load_index_text("file.txt".into()).await.unwrap()
    );

    uncommitted_diff.update(cx, |diff, cx| {
        let expected_hunks = hunks
            .iter()
            .map(|hunk| (hunk.range.start.row, hunk.secondary_status))
            .collect::<Vec<_>>();
        let actual_hunks = diff
            .hunks(&snapshot, cx)
            .map(|hunk| (hunk.range.start.row, hunk.secondary_status))
            .collect::<Vec<_>>();
        assert_eq!(actual_hunks, expected_hunks);
    });
}

#[gpui::test]
async fn test_single_file_diffs(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let committed_contents = r#"
        fn main() {
            println!("hello from HEAD");
        }
    "#
    .unindent();
    let file_contents = r#"
        fn main() {
            println!("hello from the working copy");
        }
    "#
    .unindent();

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/dir",
        json!({
            ".git": {},
           "src": {
               "main.rs": file_contents,
           }
        }),
    )
    .await;

    fs.set_head_for_repo(
        Path::new("/dir/.git"),
        &[("src/main.rs".into(), committed_contents.clone())],
        "deadbeef",
    );
    fs.set_index_for_repo(
        Path::new("/dir/.git"),
        &[("src/main.rs".into(), committed_contents.clone())],
    );

    let project = Project::test(fs.clone(), ["/dir/src/main.rs".as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/src/main.rs", cx)
        })
        .await
        .unwrap();
    let uncommitted_diff = project
        .update(cx, |project, cx| {
            project.open_uncommitted_diff(buffer.clone(), cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();
    uncommitted_diff.update(cx, |uncommitted_diff, cx| {
        let snapshot = buffer.read(cx).snapshot();
        assert_hunks(
            uncommitted_diff.hunks(&snapshot, cx),
            &snapshot,
            &uncommitted_diff.base_text_string().unwrap(),
            &[(
                1..2,
                "    println!(\"hello from HEAD\");\n",
                "    println!(\"hello from the working copy\");\n",
                DiffHunkStatus {
                    kind: DiffHunkStatusKind::Modified,
                    secondary: DiffHunkSecondaryStatus::HasSecondaryHunk,
                },
            )],
        );
    });
}

#[gpui::test]
async fn test_repository_and_path_for_project_path(
    background_executor: BackgroundExecutor,
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(background_executor);
    fs.insert_tree(
        path!("/root"),
        json!({
            "c.txt": "",
            "dir1": {
                ".git": {},
                "deps": {
                    "dep1": {
                        ".git": {},
                        "src": {
                            "a.txt": ""
                        }
                    }
                },
                "src": {
                    "b.txt": ""
                }
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    let tree_id = tree.read_with(cx, |tree, _| tree.id());
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.run_until_parked();

    project.read_with(cx, |project, cx| {
        let git_store = project.git_store().read(cx);
        let pairs = [
            ("c.txt", None),
            ("dir1/src/b.txt", Some((path!("/root/dir1"), "src/b.txt"))),
            (
                "dir1/deps/dep1/src/a.txt",
                Some((path!("/root/dir1/deps/dep1"), "src/a.txt")),
            ),
        ];
        let expected = pairs
            .iter()
            .map(|(path, result)| {
                (
                    path,
                    result.map(|(repo, repo_path)| {
                        (Path::new(repo).into(), RepoPath::from(repo_path))
                    }),
                )
            })
            .collect::<Vec<_>>();
        let actual = pairs
            .iter()
            .map(|(path, _)| {
                let project_path = (tree_id, Path::new(path)).into();
                let result = maybe!({
                    let (repo, repo_path) =
                        git_store.repository_and_path_for_project_path(&project_path, cx)?;
                    Some((repo.read(cx).work_directory_abs_path.clone(), repo_path))
                });
                (path, result)
            })
            .collect::<Vec<_>>();
        pretty_assertions::assert_eq!(expected, actual);
    });

    fs.remove_dir(path!("/root/dir1/.git").as_ref(), RemoveOptions::default())
        .await
        .unwrap();
    cx.run_until_parked();

    project.read_with(cx, |project, cx| {
        let git_store = project.git_store().read(cx);
        assert_eq!(
            git_store.repository_and_path_for_project_path(
                &(tree_id, Path::new("dir1/src/b.txt")).into(),
                cx
            ),
            None
        );
    });
}

#[gpui::test]
async fn test_home_dir_as_git_repository(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        path!("/root"),
        json!({
            "home": {
                ".git": {},
                "project": {
                    "a.txt": "A"
                },
            },
        }),
    )
    .await;
    fs.set_home_dir(Path::new(path!("/root/home")).to_owned());

    let project = Project::test(fs.clone(), [path!("/root/home/project").as_ref()], cx).await;
    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    let tree_id = tree.read_with(cx, |tree, _| tree.id());

    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    tree.flush_fs_events(cx).await;

    project.read_with(cx, |project, cx| {
        let containing = project
            .git_store()
            .read(cx)
            .repository_and_path_for_project_path(&(tree_id, "a.txt").into(), cx);
        assert!(containing.is_none());
    });

    let project = Project::test(fs.clone(), [path!("/root/home").as_ref()], cx).await;
    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    let tree_id = tree.read_with(cx, |tree, _| tree.id());
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    tree.flush_fs_events(cx).await;

    project.read_with(cx, |project, cx| {
        let containing = project
            .git_store()
            .read(cx)
            .repository_and_path_for_project_path(&(tree_id, "project/a.txt").into(), cx);
        assert_eq!(
            containing
                .unwrap()
                .0
                .read(cx)
                .work_directory_abs_path
                .as_ref(),
            Path::new(path!("/root/home"))
        );
    });
}

#[gpui::test]
async fn test_git_repository_status(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let root = TempTree::new(json!({
        "project": {
            "a.txt": "a",    // Modified
            "b.txt": "bb",   // Added
            "c.txt": "ccc",  // Unchanged
            "d.txt": "dddd", // Deleted
        },
    }));

    // Set up git repository before creating the project.
    let work_dir = root.path().join("project");
    let repo = git_init(work_dir.as_path());
    git_add("a.txt", &repo);
    git_add("c.txt", &repo);
    git_add("d.txt", &repo);
    git_commit("Initial commit", &repo);
    std::fs::remove_file(work_dir.join("d.txt")).unwrap();
    std::fs::write(work_dir.join("a.txt"), "aa").unwrap();

    let project = Project::test(
        Arc::new(RealFs::new(None, cx.executor())),
        [root.path()],
        cx,
    )
    .await;

    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    let repository = project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });

    // Check that the right git state is observed on startup
    repository.read_with(cx, |repository, _| {
        let entries = repository.cached_status().collect::<Vec<_>>();
        assert_eq!(
            entries,
            [
                StatusEntry {
                    repo_path: "a.txt".into(),
                    status: StatusCode::Modified.worktree(),
                },
                StatusEntry {
                    repo_path: "b.txt".into(),
                    status: FileStatus::Untracked,
                },
                StatusEntry {
                    repo_path: "d.txt".into(),
                    status: StatusCode::Deleted.worktree(),
                },
            ]
        );
    });

    std::fs::write(work_dir.join("c.txt"), "some changes").unwrap();

    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    repository.read_with(cx, |repository, _| {
        let entries = repository.cached_status().collect::<Vec<_>>();
        assert_eq!(
            entries,
            [
                StatusEntry {
                    repo_path: "a.txt".into(),
                    status: StatusCode::Modified.worktree(),
                },
                StatusEntry {
                    repo_path: "b.txt".into(),
                    status: FileStatus::Untracked,
                },
                StatusEntry {
                    repo_path: "c.txt".into(),
                    status: StatusCode::Modified.worktree(),
                },
                StatusEntry {
                    repo_path: "d.txt".into(),
                    status: StatusCode::Deleted.worktree(),
                },
            ]
        );
    });

    git_add("a.txt", &repo);
    git_add("c.txt", &repo);
    git_remove_index(Path::new("d.txt"), &repo);
    git_commit("Another commit", &repo);
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    std::fs::remove_file(work_dir.join("a.txt")).unwrap();
    std::fs::remove_file(work_dir.join("b.txt")).unwrap();
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    repository.read_with(cx, |repository, _cx| {
        let entries = repository.cached_status().collect::<Vec<_>>();

        // Deleting an untracked entry, b.txt, should leave no status
        // a.txt was tracked, and so should have a status
        assert_eq!(
            entries,
            [StatusEntry {
                repo_path: "a.txt".into(),
                status: StatusCode::Deleted.worktree(),
            }]
        );
    });
}

#[gpui::test]
async fn test_git_status_postprocessing(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let root = TempTree::new(json!({
        "project": {
            "sub": {},
            "a.txt": "",
        },
    }));

    let work_dir = root.path().join("project");
    let repo = git_init(work_dir.as_path());
    // a.txt exists in HEAD and the working copy but is deleted in the index.
    git_add("a.txt", &repo);
    git_commit("Initial commit", &repo);
    git_remove_index("a.txt".as_ref(), &repo);
    // `sub` is a nested git repository.
    let _sub = git_init(&work_dir.join("sub"));

    let project = Project::test(
        Arc::new(RealFs::new(None, cx.executor())),
        [root.path()],
        cx,
    )
    .await;

    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    let repository = project.read_with(cx, |project, cx| {
        project
            .repositories(cx)
            .values()
            .find(|repo| repo.read(cx).work_directory_abs_path.ends_with("project"))
            .unwrap()
            .clone()
    });

    repository.read_with(cx, |repository, _cx| {
        let entries = repository.cached_status().collect::<Vec<_>>();

        // `sub` doesn't appear in our computed statuses.
        // a.txt appears with a combined `DA` status.
        assert_eq!(
            entries,
            [StatusEntry {
                repo_path: "a.txt".into(),
                status: TrackedStatus {
                    index_status: StatusCode::Deleted,
                    worktree_status: StatusCode::Added
                }
                .into(),
            }]
        )
    });
}

#[gpui::test]
async fn test_repository_subfolder_git_status(
    executor: gpui::BackgroundExecutor,
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor);
    fs.insert_tree(
        path!("/root"),
        json!({
            "my-repo": {
                ".git": {},
                "a.txt": "a",
                "sub-folder-1": {
                    "sub-folder-2": {
                        "c.txt": "cc",
                        "d": {
                            "e.txt": "eee"
                        }
                    },
                }
            },
        }),
    )
    .await;

    const C_TXT: &str = "sub-folder-1/sub-folder-2/c.txt";
    const E_TXT: &str = "sub-folder-1/sub-folder-2/d/e.txt";

    fs.set_status_for_repo(
        path!("/root/my-repo/.git").as_ref(),
        &[(E_TXT.as_ref(), FileStatus::Untracked)],
    );

    let project = Project::test(
        fs.clone(),
        [path!("/root/my-repo/sub-folder-1/sub-folder-2").as_ref()],
        cx,
    )
    .await;

    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.run_until_parked();

    let repository = project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });

    // Ensure that the git status is loaded correctly
    repository.read_with(cx, |repository, _cx| {
        assert_eq!(
            repository.work_directory_abs_path,
            Path::new(path!("/root/my-repo")).into()
        );

        assert_eq!(repository.status_for_path(&C_TXT.into()), None);
        assert_eq!(
            repository.status_for_path(&E_TXT.into()).unwrap().status,
            FileStatus::Untracked
        );
    });

    fs.set_status_for_repo(path!("/root/my-repo/.git").as_ref(), &[]);
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.run_until_parked();

    repository.read_with(cx, |repository, _cx| {
        assert_eq!(repository.status_for_path(&C_TXT.into()), None);
        assert_eq!(repository.status_for_path(&E_TXT.into()), None);
    });
}

// TODO: this test is flaky (especially on Windows but at least sometimes on all platforms).
#[cfg(any())]
#[gpui::test]
async fn test_conflicted_cherry_pick(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let root = TempTree::new(json!({
        "project": {
            "a.txt": "a",
        },
    }));
    let root_path = root.path();

    let repo = git_init(&root_path.join("project"));
    git_add("a.txt", &repo);
    git_commit("init", &repo);

    let project = Project::test(Arc::new(RealFs::new(None, cx.executor())), [root_path], cx).await;

    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    let repository = project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });

    git_branch("other-branch", &repo);
    git_checkout("refs/heads/other-branch", &repo);
    std::fs::write(root_path.join("project/a.txt"), "A").unwrap();
    git_add("a.txt", &repo);
    git_commit("capitalize", &repo);
    let commit = repo
        .head()
        .expect("Failed to get HEAD")
        .peel_to_commit()
        .expect("HEAD is not a commit");
    git_checkout("refs/heads/main", &repo);
    std::fs::write(root_path.join("project/a.txt"), "b").unwrap();
    git_add("a.txt", &repo);
    git_commit("improve letter", &repo);
    git_cherry_pick(&commit, &repo);
    std::fs::read_to_string(root_path.join("project/.git/CHERRY_PICK_HEAD"))
        .expect("No CHERRY_PICK_HEAD");
    pretty_assertions::assert_eq!(
        git_status(&repo),
        collections::HashMap::from_iter([("a.txt".to_owned(), git2::Status::CONFLICTED)])
    );
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();
    let conflicts = repository.update(cx, |repository, _| {
        repository
            .merge_conflicts
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(conflicts, [RepoPath::from("a.txt")]);

    git_add("a.txt", &repo);
    // Attempt to manually simulate what `git cherry-pick --continue` would do.
    git_commit("whatevs", &repo);
    std::fs::remove_file(root.path().join("project/.git/CHERRY_PICK_HEAD"))
        .expect("Failed to remove CHERRY_PICK_HEAD");
    pretty_assertions::assert_eq!(git_status(&repo), collections::HashMap::default());
    tree.flush_fs_events(cx).await;
    let conflicts = repository.update(cx, |repository, _| {
        repository
            .merge_conflicts
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(conflicts, []);
}

#[gpui::test]
async fn test_update_gitignore(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        path!("/root"),
        json!({
            ".git": {},
            ".gitignore": "*.txt\n",
            "a.xml": "<a></a>",
            "b.txt": "Some text"
        }),
    )
    .await;

    fs.set_head_and_index_for_repo(
        path!("/root/.git").as_ref(),
        &[
            (".gitignore".into(), "*.txt\n".into()),
            ("a.xml".into(), "<a></a>".into()),
        ],
    );

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    let repository = project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });

    // One file is unmodified, the other is ignored.
    cx.read(|cx| {
        assert_entry_git_state(tree.read(cx), repository.read(cx), "a.xml", None, false);
        assert_entry_git_state(tree.read(cx), repository.read(cx), "b.txt", None, true);
    });

    // Change the gitignore, and stage the newly non-ignored file.
    fs.atomic_write(path!("/root/.gitignore").into(), "*.xml\n".into())
        .await
        .unwrap();
    fs.set_index_for_repo(
        Path::new(path!("/root/.git")),
        &[
            (".gitignore".into(), "*.txt\n".into()),
            ("a.xml".into(), "<a></a>".into()),
            ("b.txt".into(), "Some text".into()),
        ],
    );

    cx.executor().run_until_parked();
    cx.read(|cx| {
        assert_entry_git_state(tree.read(cx), repository.read(cx), "a.xml", None, true);
        assert_entry_git_state(
            tree.read(cx),
            repository.read(cx),
            "b.txt",
            Some(StatusCode::Added),
            false,
        );
    });
}

// NOTE:
// This test always fails on Windows, because on Windows, unlike on Unix, you can't rename
// a directory which some program has already open.
// This is a limitation of the Windows.
// See: https://stackoverflow.com/questions/41365318/access-is-denied-when-renaming-folder
#[gpui::test]
#[cfg_attr(target_os = "windows", ignore)]
async fn test_rename_work_directory(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let root = TempTree::new(json!({
        "projects": {
            "project1": {
                "a": "",
                "b": "",
            }
        },

    }));
    let root_path = root.path();

    let repo = git_init(&root_path.join("projects/project1"));
    git_add("a", &repo);
    git_commit("init", &repo);
    std::fs::write(root_path.join("projects/project1/a"), "aa").unwrap();

    let project = Project::test(Arc::new(RealFs::new(None, cx.executor())), [root_path], cx).await;

    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    let repository = project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });

    repository.read_with(cx, |repository, _| {
        assert_eq!(
            repository.work_directory_abs_path.as_ref(),
            root_path.join("projects/project1").as_path()
        );
        assert_eq!(
            repository
                .status_for_path(&"a".into())
                .map(|entry| entry.status),
            Some(StatusCode::Modified.worktree()),
        );
        assert_eq!(
            repository
                .status_for_path(&"b".into())
                .map(|entry| entry.status),
            Some(FileStatus::Untracked),
        );
    });

    std::fs::rename(
        root_path.join("projects/project1"),
        root_path.join("projects/project2"),
    )
    .unwrap();
    tree.flush_fs_events(cx).await;

    repository.read_with(cx, |repository, _| {
        assert_eq!(
            repository.work_directory_abs_path.as_ref(),
            root_path.join("projects/project2").as_path()
        );
        assert_eq!(
            repository.status_for_path(&"a".into()).unwrap().status,
            StatusCode::Modified.worktree(),
        );
        assert_eq!(
            repository.status_for_path(&"b".into()).unwrap().status,
            FileStatus::Untracked,
        );
    });
}

// NOTE: This test always fails on Windows, because on Windows, unlike on Unix,
// you can't rename a directory which some program has already open. This is a
// limitation of the Windows. See:
// https://stackoverflow.com/questions/41365318/access-is-denied-when-renaming-folder
#[gpui::test]
#[cfg_attr(target_os = "windows", ignore)]
async fn test_file_status(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    const IGNORE_RULE: &str = "**/target";

    let root = TempTree::new(json!({
        "project": {
            "a.txt": "a",
            "b.txt": "bb",
            "c": {
                "d": {
                    "e.txt": "eee"
                }
            },
            "f.txt": "ffff",
            "target": {
                "build_file": "???"
            },
            ".gitignore": IGNORE_RULE
        },

    }));
    let root_path = root.path();

    const A_TXT: &str = "a.txt";
    const B_TXT: &str = "b.txt";
    const E_TXT: &str = "c/d/e.txt";
    const F_TXT: &str = "f.txt";
    const DOTGITIGNORE: &str = ".gitignore";
    const BUILD_FILE: &str = "target/build_file";

    // Set up git repository before creating the worktree.
    let work_dir = root.path().join("project");
    let mut repo = git_init(work_dir.as_path());
    repo.add_ignore_rule(IGNORE_RULE).unwrap();
    git_add(A_TXT, &repo);
    git_add(E_TXT, &repo);
    git_add(DOTGITIGNORE, &repo);
    git_commit("Initial commit", &repo);

    let project = Project::test(Arc::new(RealFs::new(None, cx.executor())), [root_path], cx).await;

    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    let repository = project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });

    // Check that the right git state is observed on startup
    repository.read_with(cx, |repository, _cx| {
        assert_eq!(
            repository.work_directory_abs_path.as_ref(),
            root_path.join("project").as_path()
        );

        assert_eq!(
            repository.status_for_path(&B_TXT.into()).unwrap().status,
            FileStatus::Untracked,
        );
        assert_eq!(
            repository.status_for_path(&F_TXT.into()).unwrap().status,
            FileStatus::Untracked,
        );
    });

    // Modify a file in the working copy.
    std::fs::write(work_dir.join(A_TXT), "aa").unwrap();
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    // The worktree detects that the file's git status has changed.
    repository.read_with(cx, |repository, _| {
        assert_eq!(
            repository.status_for_path(&A_TXT.into()).unwrap().status,
            StatusCode::Modified.worktree(),
        );
    });

    // Create a commit in the git repository.
    git_add(A_TXT, &repo);
    git_add(B_TXT, &repo);
    git_commit("Committing modified and added", &repo);
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    // The worktree detects that the files' git status have changed.
    repository.read_with(cx, |repository, _cx| {
        assert_eq!(
            repository.status_for_path(&F_TXT.into()).unwrap().status,
            FileStatus::Untracked,
        );
        assert_eq!(repository.status_for_path(&B_TXT.into()), None);
        assert_eq!(repository.status_for_path(&A_TXT.into()), None);
    });

    // Modify files in the working copy and perform git operations on other files.
    git_reset(0, &repo);
    git_remove_index(Path::new(B_TXT), &repo);
    git_stash(&mut repo);
    std::fs::write(work_dir.join(E_TXT), "eeee").unwrap();
    std::fs::write(work_dir.join(BUILD_FILE), "this should be ignored").unwrap();
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    // Check that more complex repo changes are tracked
    repository.read_with(cx, |repository, _cx| {
        assert_eq!(repository.status_for_path(&A_TXT.into()), None);
        assert_eq!(
            repository.status_for_path(&B_TXT.into()).unwrap().status,
            FileStatus::Untracked,
        );
        assert_eq!(
            repository.status_for_path(&E_TXT.into()).unwrap().status,
            StatusCode::Modified.worktree(),
        );
    });

    std::fs::remove_file(work_dir.join(B_TXT)).unwrap();
    std::fs::remove_dir_all(work_dir.join("c")).unwrap();
    std::fs::write(
        work_dir.join(DOTGITIGNORE),
        [IGNORE_RULE, "f.txt"].join("\n"),
    )
    .unwrap();

    git_add(Path::new(DOTGITIGNORE), &repo);
    git_commit("Committing modified git ignore", &repo);

    tree.flush_fs_events(cx).await;
    cx.executor().run_until_parked();

    let mut renamed_dir_name = "first_directory/second_directory";
    const RENAMED_FILE: &str = "rf.txt";

    std::fs::create_dir_all(work_dir.join(renamed_dir_name)).unwrap();
    std::fs::write(
        work_dir.join(renamed_dir_name).join(RENAMED_FILE),
        "new-contents",
    )
    .unwrap();

    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    repository.read_with(cx, |repository, _cx| {
        assert_eq!(
            repository
                .status_for_path(&Path::new(renamed_dir_name).join(RENAMED_FILE).into())
                .unwrap()
                .status,
            FileStatus::Untracked,
        );
    });

    renamed_dir_name = "new_first_directory/second_directory";

    std::fs::rename(
        work_dir.join("first_directory"),
        work_dir.join("new_first_directory"),
    )
    .unwrap();

    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    repository.read_with(cx, |repository, _cx| {
        assert_eq!(
            repository
                .status_for_path(&Path::new(renamed_dir_name).join(RENAMED_FILE).into())
                .unwrap()
                .status,
            FileStatus::Untracked,
        );
    });
}

#[gpui::test]
async fn test_repos_in_invisible_worktrees(
    executor: BackgroundExecutor,
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(executor);
    fs.insert_tree(
        path!("/root"),
        json!({
            "dir1": {
                ".git": {},
                "dep1": {
                    ".git": {},
                    "src": {
                        "a.txt": "",
                    },
                },
                "b.txt": "",
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root/dir1/dep1").as_ref()], cx).await;
    let _visible_worktree =
        project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;

    let repos = project.read_with(cx, |project, cx| {
        project
            .repositories(cx)
            .values()
            .map(|repo| repo.read(cx).work_directory_abs_path.clone())
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(repos, [Path::new(path!("/root/dir1/dep1")).into()]);

    let (_invisible_worktree, _) = project
        .update(cx, |project, cx| {
            project.worktree_store.update(cx, |worktree_store, cx| {
                worktree_store.find_or_create_worktree(path!("/root/dir1/b.txt"), false, cx)
            })
        })
        .await
        .expect("failed to create worktree");
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;

    let repos = project.read_with(cx, |project, cx| {
        project
            .repositories(cx)
            .values()
            .map(|repo| repo.read(cx).work_directory_abs_path.clone())
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(repos, [Path::new(path!("/root/dir1/dep1")).into()]);
}

#[gpui::test(iterations = 10)]
async fn test_rescan_with_gitignore(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings::<WorktreeSettings>(cx, |project_settings| {
                project_settings.file_scan_exclusions = Some(Vec::new());
            });
        });
    });
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        path!("/root"),
        json!({
            ".gitignore": "ancestor-ignored-file1\nancestor-ignored-file2\n",
            "tree": {
                ".git": {},
                ".gitignore": "ignored-dir\n",
                "tracked-dir": {
                    "tracked-file1": "",
                    "ancestor-ignored-file1": "",
                },
                "ignored-dir": {
                    "ignored-file1": ""
                }
            }
        }),
    )
    .await;
    fs.set_head_and_index_for_repo(
        path!("/root/tree/.git").as_ref(),
        &[
            (".gitignore".into(), "ignored-dir\n".into()),
            ("tracked-dir/tracked-file1".into(), "".into()),
        ],
    );

    let project = Project::test(fs.clone(), [path!("/root/tree").as_ref()], cx).await;

    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    let repository = project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });

    tree.read_with(cx, |tree, _| {
        tree.as_local()
            .unwrap()
            .manually_refresh_entries_for_paths(vec![Path::new("ignored-dir").into()])
    })
    .recv()
    .await;

    cx.read(|cx| {
        assert_entry_git_state(
            tree.read(cx),
            repository.read(cx),
            "tracked-dir/tracked-file1",
            None,
            false,
        );
        assert_entry_git_state(
            tree.read(cx),
            repository.read(cx),
            "tracked-dir/ancestor-ignored-file1",
            None,
            false,
        );
        assert_entry_git_state(
            tree.read(cx),
            repository.read(cx),
            "ignored-dir/ignored-file1",
            None,
            true,
        );
    });

    fs.create_file(
        path!("/root/tree/tracked-dir/tracked-file2").as_ref(),
        Default::default(),
    )
    .await
    .unwrap();
    fs.set_index_for_repo(
        path!("/root/tree/.git").as_ref(),
        &[
            (".gitignore".into(), "ignored-dir\n".into()),
            ("tracked-dir/tracked-file1".into(), "".into()),
            ("tracked-dir/tracked-file2".into(), "".into()),
        ],
    );
    fs.create_file(
        path!("/root/tree/tracked-dir/ancestor-ignored-file2").as_ref(),
        Default::default(),
    )
    .await
    .unwrap();
    fs.create_file(
        path!("/root/tree/ignored-dir/ignored-file2").as_ref(),
        Default::default(),
    )
    .await
    .unwrap();

    cx.executor().run_until_parked();
    cx.read(|cx| {
        assert_entry_git_state(
            tree.read(cx),
            repository.read(cx),
            "tracked-dir/tracked-file2",
            Some(StatusCode::Added),
            false,
        );
        assert_entry_git_state(
            tree.read(cx),
            repository.read(cx),
            "tracked-dir/ancestor-ignored-file2",
            None,
            false,
        );
        assert_entry_git_state(
            tree.read(cx),
            repository.read(cx),
            "ignored-dir/ignored-file2",
            None,
            true,
        );
        assert!(tree.read(cx).entry_for_path(".git").unwrap().is_ignored);
    });
}

#[gpui::test]
async fn test_git_worktrees_and_submodules(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            ".git": {
                "worktrees": {
                    "some-worktree": {
                        "commondir": "../..\n",
                        // For is_git_dir
                        "HEAD": "",
                        "config": ""
                    }
                },
                "modules": {
                    "subdir": {
                        "some-submodule": {
                            // For is_git_dir
                            "HEAD": "",
                            "config": "",
                        }
                    }
                }
            },
            "src": {
                "a.txt": "A",
            },
            "some-worktree": {
                ".git": "gitdir: ../.git/worktrees/some-worktree\n",
                "src": {
                    "b.txt": "B",
                }
            },
            "subdir": {
                "some-submodule": {
                    ".git": "gitdir: ../../.git/modules/subdir/some-submodule\n",
                    "c.txt": "C",
                }
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let scan_complete = project.update(cx, |project, cx| project.git_scans_complete(cx));
    scan_complete.await;

    let mut repositories = project.update(cx, |project, cx| {
        project
            .repositories(cx)
            .values()
            .map(|repo| repo.read(cx).work_directory_abs_path.clone())
            .collect::<Vec<_>>()
    });
    repositories.sort();
    pretty_assertions::assert_eq!(
        repositories,
        [
            Path::new(path!("/project")).into(),
            Path::new(path!("/project/some-worktree")).into(),
            Path::new(path!("/project/subdir/some-submodule")).into(),
        ]
    );

    // Generate a git-related event for the worktree and check that it's refreshed.
    fs.with_git_state(
        path!("/project/some-worktree/.git").as_ref(),
        true,
        |state| {
            state
                .head_contents
                .insert("src/b.txt".into(), "b".to_owned());
            state
                .index_contents
                .insert("src/b.txt".into(), "b".to_owned());
        },
    )
    .unwrap();
    cx.run_until_parked();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/project/some-worktree/src/b.txt"), cx)
        })
        .await
        .unwrap();
    let (worktree_repo, barrier) = project.update(cx, |project, cx| {
        let (repo, _) = project
            .git_store()
            .read(cx)
            .repository_and_path_for_buffer_id(buffer.read(cx).remote_id(), cx)
            .unwrap();
        pretty_assertions::assert_eq!(
            repo.read(cx).work_directory_abs_path,
            Path::new(path!("/project/some-worktree")).into(),
        );
        let barrier = repo.update(cx, |repo, _| repo.barrier());
        (repo.clone(), barrier)
    });
    barrier.await.unwrap();
    worktree_repo.update(cx, |repo, _| {
        pretty_assertions::assert_eq!(
            repo.status_for_path(&"src/b.txt".into()).unwrap().status,
            StatusCode::Modified.worktree(),
        );
    });

    // The same for the submodule.
    fs.with_git_state(
        path!("/project/subdir/some-submodule/.git").as_ref(),
        true,
        |state| {
            state.head_contents.insert("c.txt".into(), "c".to_owned());
            state.index_contents.insert("c.txt".into(), "c".to_owned());
        },
    )
    .unwrap();
    cx.run_until_parked();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/project/subdir/some-submodule/c.txt"), cx)
        })
        .await
        .unwrap();
    let (submodule_repo, barrier) = project.update(cx, |project, cx| {
        let (repo, _) = project
            .git_store()
            .read(cx)
            .repository_and_path_for_buffer_id(buffer.read(cx).remote_id(), cx)
            .unwrap();
        pretty_assertions::assert_eq!(
            repo.read(cx).work_directory_abs_path,
            Path::new(path!("/project/subdir/some-submodule")).into(),
        );
        let barrier = repo.update(cx, |repo, _| repo.barrier());
        (repo.clone(), barrier)
    });
    barrier.await.unwrap();
    submodule_repo.update(cx, |repo, _| {
        pretty_assertions::assert_eq!(
            repo.status_for_path(&"c.txt".into()).unwrap().status,
            StatusCode::Modified.worktree(),
        );
    });
}

#[gpui::test]
async fn test_repository_deduplication(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        path!("/root"),
        json!({
            "project": {
                ".git": {},
                "child1": {
                    "a.txt": "A",
                },
                "child2": {
                    "b.txt": "B",
                }
            }
        }),
    )
    .await;

    let project = Project::test(
        fs.clone(),
        [
            path!("/root/project/child1").as_ref(),
            path!("/root/project/child2").as_ref(),
        ],
        cx,
    )
    .await;

    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    tree.flush_fs_events(cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.executor().run_until_parked();

    let repos = project.read_with(cx, |project, cx| {
        project
            .repositories(cx)
            .values()
            .map(|repo| repo.read(cx).work_directory_abs_path.clone())
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(repos, [Path::new(path!("/root/project")).into()]);
}

async fn search(
    project: &Entity<Project>,
    query: SearchQuery,
    cx: &mut gpui::TestAppContext,
) -> Result<HashMap<String, Vec<Range<usize>>>> {
    let search_rx = project.update(cx, |project, cx| project.search(query, cx));
    let mut results = HashMap::default();
    while let Ok(search_result) = search_rx.recv().await {
        match search_result {
            SearchResult::Buffer { buffer, ranges } => {
                results.entry(buffer).or_insert(ranges);
            }
            SearchResult::LimitReached => {}
        }
    }
    Ok(results
        .into_iter()
        .map(|(buffer, ranges)| {
            buffer.update(cx, |buffer, cx| {
                let path = buffer
                    .file()
                    .unwrap()
                    .full_path(cx)
                    .to_string_lossy()
                    .to_string();
                let ranges = ranges
                    .into_iter()
                    .map(|range| range.to_offset(buffer))
                    .collect::<Vec<_>>();
                (path, ranges)
            })
        })
        .collect())
}

pub fn init_test(cx: &mut gpui::TestAppContext) {
    zlog::init_test();

    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        release_channel::init(SemanticVersion::default(), cx);
        language::init(cx);
        Project::init_settings(cx);
    });
}

fn json_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "JSON".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["json".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        None,
    ))
}

fn js_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["js".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        None,
    ))
}

fn rust_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ))
}

fn typescript_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ts".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
    ))
}

fn tsx_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "tsx".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["tsx".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
    ))
}

fn get_all_tasks(
    project: &Entity<Project>,
    task_contexts: Arc<TaskContexts>,
    cx: &mut App,
) -> Task<Vec<(TaskSourceKind, ResolvedTask)>> {
    let new_tasks = project.update(cx, |project, cx| {
        project.task_store.update(cx, |task_store, cx| {
            task_store.task_inventory().unwrap().update(cx, |this, cx| {
                this.used_and_current_resolved_tasks(task_contexts, cx)
            })
        })
    });

    cx.background_spawn(async move {
        let (mut old, new) = new_tasks.await;
        old.extend(new);
        old
    })
}

#[track_caller]
fn assert_entry_git_state(
    tree: &Worktree,
    repository: &Repository,
    path: &str,
    index_status: Option<StatusCode>,
    is_ignored: bool,
) {
    assert_eq!(tree.abs_path(), repository.work_directory_abs_path);
    let entry = tree
        .entry_for_path(path)
        .unwrap_or_else(|| panic!("entry {path} not found"));
    let status = repository
        .status_for_path(&path.into())
        .map(|entry| entry.status);
    let expected = index_status.map(|index_status| {
        TrackedStatus {
            index_status,
            worktree_status: StatusCode::Unmodified,
        }
        .into()
    });
    assert_eq!(
        status, expected,
        "expected {path} to have git status: {expected:?}"
    );
    assert_eq!(
        entry.is_ignored, is_ignored,
        "expected {path} to have is_ignored: {is_ignored}"
    );
}

#[track_caller]
fn git_init(path: &Path) -> git2::Repository {
    let mut init_opts = RepositoryInitOptions::new();
    init_opts.initial_head("main");
    git2::Repository::init_opts(path, &init_opts).expect("Failed to initialize git repository")
}

#[track_caller]
fn git_add<P: AsRef<Path>>(path: P, repo: &git2::Repository) {
    let path = path.as_ref();
    let mut index = repo.index().expect("Failed to get index");
    index.add_path(path).expect("Failed to add file");
    index.write().expect("Failed to write index");
}

#[track_caller]
fn git_remove_index(path: &Path, repo: &git2::Repository) {
    let mut index = repo.index().expect("Failed to get index");
    index.remove_path(path).expect("Failed to add file");
    index.write().expect("Failed to write index");
}

#[track_caller]
fn git_commit(msg: &'static str, repo: &git2::Repository) {
    use git2::Signature;

    let signature = Signature::now("test", "test@zed.dev").unwrap();
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    if let Ok(head) = repo.head() {
        let parent_obj = head.peel(git2::ObjectType::Commit).unwrap();

        let parent_commit = parent_obj.as_commit().unwrap();

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            msg,
            &tree,
            &[parent_commit],
        )
        .expect("Failed to commit with parent");
    } else {
        repo.commit(Some("HEAD"), &signature, &signature, msg, &tree, &[])
            .expect("Failed to commit");
    }
}

#[cfg(any())]
#[track_caller]
fn git_cherry_pick(commit: &git2::Commit<'_>, repo: &git2::Repository) {
    repo.cherrypick(commit, None).expect("Failed to cherrypick");
}

#[track_caller]
fn git_stash(repo: &mut git2::Repository) {
    use git2::Signature;

    let signature = Signature::now("test", "test@zed.dev").unwrap();
    repo.stash_save(&signature, "N/A", None)
        .expect("Failed to stash");
}

#[track_caller]
fn git_reset(offset: usize, repo: &git2::Repository) {
    let head = repo.head().expect("Couldn't get repo head");
    let object = head.peel(git2::ObjectType::Commit).unwrap();
    let commit = object.as_commit().unwrap();
    let new_head = commit
        .parents()
        .inspect(|parnet| {
            parnet.message();
        })
        .nth(offset)
        .expect("Not enough history");
    repo.reset(new_head.as_object(), git2::ResetType::Soft, None)
        .expect("Could not reset");
}

#[cfg(any())]
#[track_caller]
fn git_branch(name: &str, repo: &git2::Repository) {
    let head = repo
        .head()
        .expect("Couldn't get repo head")
        .peel_to_commit()
        .expect("HEAD is not a commit");
    repo.branch(name, &head, false).expect("Failed to commit");
}

#[cfg(any())]
#[track_caller]
fn git_checkout(name: &str, repo: &git2::Repository) {
    repo.set_head(name).expect("Failed to set head");
    repo.checkout_head(None).expect("Failed to check out head");
}

#[cfg(any())]
#[track_caller]
fn git_status(repo: &git2::Repository) -> collections::HashMap<String, git2::Status> {
    repo.statuses(None)
        .unwrap()
        .iter()
        .map(|status| (status.path().unwrap().to_string(), status.status()))
        .collect()
}

#[gpui::test]
async fn test_find_project_path_abs(
    background_executor: BackgroundExecutor,
    cx: &mut gpui::TestAppContext,
) {
    // find_project_path should work with absolute paths
    init_test(cx);

    let fs = FakeFs::new(background_executor);
    fs.insert_tree(
        path!("/root"),
        json!({
            "project1": {
                "file1.txt": "content1",
                "subdir": {
                    "file2.txt": "content2"
                }
            },
            "project2": {
                "file3.txt": "content3"
            }
        }),
    )
    .await;

    let project = Project::test(
        fs.clone(),
        [
            path!("/root/project1").as_ref(),
            path!("/root/project2").as_ref(),
        ],
        cx,
    )
    .await;

    // Make sure the worktrees are fully initialized
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.run_until_parked();

    let (project1_abs_path, project1_id, project2_abs_path, project2_id) =
        project.read_with(cx, |project, cx| {
            let worktrees: Vec<_> = project.worktrees(cx).collect();
            let abs_path1 = worktrees[0].read(cx).abs_path().to_path_buf();
            let id1 = worktrees[0].read(cx).id();
            let abs_path2 = worktrees[1].read(cx).abs_path().to_path_buf();
            let id2 = worktrees[1].read(cx).id();
            (abs_path1, id1, abs_path2, id2)
        });

    project.update(cx, |project, cx| {
        let abs_path = project1_abs_path.join("file1.txt");
        let found_path = project.find_project_path(abs_path, cx).unwrap();
        assert_eq!(found_path.worktree_id, project1_id);
        assert_eq!(found_path.path.as_ref(), Path::new("file1.txt"));

        let abs_path = project1_abs_path.join("subdir").join("file2.txt");
        let found_path = project.find_project_path(abs_path, cx).unwrap();
        assert_eq!(found_path.worktree_id, project1_id);
        assert_eq!(found_path.path.as_ref(), Path::new("subdir/file2.txt"));

        let abs_path = project2_abs_path.join("file3.txt");
        let found_path = project.find_project_path(abs_path, cx).unwrap();
        assert_eq!(found_path.worktree_id, project2_id);
        assert_eq!(found_path.path.as_ref(), Path::new("file3.txt"));

        let abs_path = project1_abs_path.join("nonexistent.txt");
        let found_path = project.find_project_path(abs_path, cx);
        assert!(
            found_path.is_some(),
            "Should find project path for nonexistent file in worktree"
        );

        // Test with an absolute path outside any worktree
        let abs_path = Path::new("/some/other/path");
        let found_path = project.find_project_path(abs_path, cx);
        assert!(
            found_path.is_none(),
            "Should not find project path for path outside any worktree"
        );
    });
}
