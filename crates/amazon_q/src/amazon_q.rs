mod amazon_q_completion_provider;

pub use amazon_q_completion_provider::*;

use anyhow::{Context as _, Result};
#[allow(unused_imports)]
use client::{Client, proto};
use collections::BTreeMap;

use futures::{AsyncBufReadExt, StreamExt, channel::mpsc, io::BufReader};
use gpui::{App, AsyncApp, Context, Entity, EntityId, Global, Task, WeakEntity, actions};
use language::{
    Anchor, Buffer, BufferSnapshot, ToOffset, language_settings::all_language_settings,
};
use postage::watch;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use smol::{
    io::AsyncWriteExt,
    process::{Child, ChildStdin, ChildStdout},
};
use std::{path::PathBuf, process::Stdio, sync::Arc};
use ui::prelude::*;
use util::ResultExt;

actions!(amazon_q, [SignOut]);

pub fn init(http: Arc<dyn HttpClient>, cx: &mut App) {
    let amazon_q = cx::new(|_| AmazonQ::Starting);

    AmazonQ::set_global(amazon_q.clone(), cx);

    let mut provider = all_language_settings(None, cx).edit_predictions.provider;
    if provider == language::language_settings::EditPredictionProvider::AmazonQ {
        amazon_q.update(cx, |amazon_q, cx| amazon_q.start(client.clone(), cx));

        cx.observe_global::<SettingsStore>(move |cx| {
            let new_provider = all_language_settings(None, cx).edit_predictions.provider;
            if new_provider != provider {
                provider = new_provider;
                if provider == language::language_settings::EditPredictionProvider::AmazonQ {
                    amazon_q.update(cx, |amazon_q, cx| amazon_q.start(client.clone(), cx));
                } else {
                    amazon_q.update(cx, |amazon_q, _cx| amazon_q.stop());
                }
            }
        })
        .detach();

        cx.on_action(|_: &SignOut, cx| {
            if let Some(amazon_q) = AmazonQ::global(cx) {
                amazon_q.update(cx, |amazon_q, _cx| amazon_q.sign_out());
            }
        });
    }
}

pub enum AmazonQ {
    Starting,
    FailedDownload { error: anyhow::Error },
    Spawned(AmazonQClient),
    Error { error: anyhow::Error },
}

#[derive(Clone)]
pub enum AccountStatus {
    Unknown,
    NeedSubscription { activate_url: String },
    Ready,
}

#[derive(Clone)]
struct AmazonQGlobal(Entity<AmazonQ>);

impl Global for AmazonQGlobal {}

impl AmazonQ {
    fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<AmazonQGlobal>()
            .map(|model| model.0.clone())
    }

    pub fn start(&mut self, client: Arc<Client>, cx: &mut Context<Self>) {
        if let Self::Starting = self {
            cx.spawn(async move |this, cx| {

            })
        }
}
