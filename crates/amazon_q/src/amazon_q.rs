mod amazon_q_completion_provider;

pub use amazon_q_completion_provider::*;

use anyhow::Result;
use collections::BTreeMap;
use gpui::{App, Context, Entity, EntityId, Global, Task, actions};
use http_client::HttpClient;
use language::{
    Anchor, Buffer, BufferSnapshot, ToOffset, language_settings::all_language_settings,
};
use postage::watch;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use std::{path::PathBuf, sync::Arc};
use text;
use ui::prelude::*;

actions!(amazon_q, [SignOut]);

pub fn init(http_client: Arc<dyn std::fmt::Debug>, cx: &mut App) {
    let amazon_q = cx.new(|_| AmazonQ::Starting);
    AmazonQ::set_global(amazon_q.clone(), cx);

    let mut provider = all_language_settings(None, cx).edit_predictions.provider;
    if provider == language::language_settings::EditPredictionProvider::AmazonQ {
        amazon_q.update(cx, |amazon_q, cx| amazon_q.start(http_client.clone(), cx));
    }

    cx.observe_global::<SettingsStore>(move |cx| {
        let new_provider = all_language_settings(None, cx).edit_predictions.provider;
        if new_provider != provider {
            provider = new_provider;
            if provider == language::language_settings::EditPredictionProvider::AmazonQ {
                amazon_q.update(cx, |amazon_q, cx| amazon_q.start(http_client.clone(), cx));
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

pub enum AmazonQ {
    Starting,
    Ready(AmazonQClient),
    Error { error: anyhow::Error },
}

#[derive(Clone)]
pub enum AccountStatus {
    Unknown,
    NeedSubscription { start_url: String },
    Ready,
}

#[derive(Clone)]
struct AmazonQGlobal(Entity<AmazonQ>);

impl Global for AmazonQGlobal {}

impl AmazonQ {
    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<AmazonQGlobal>()
            .map(|model| model.0.clone())
    }

    fn set_global(amazon_q: Entity<Self>, cx: &mut App) {
        cx.set_global(AmazonQGlobal(amazon_q));
    }

    pub fn start(&mut self, http_client: Arc<dyn std::fmt::Debug>, cx: &mut Context<Self>) {
        if let Self::Starting = self {
            cx.spawn(async move |this, cx| {
                // Initialize AmazonQClient here
                this.update(cx, |this, cx| {
                    if let Self::Starting = this {
                        *this = Self::Spawned(AmazonQClient::new(http_client, cx)?);
                    }
                    anyhow::Ok(())
                })
            })
            .detach();
        }
    }

    pub fn stop(&mut self) {
        *self = Self::Starting;
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Spawned { .. })
    }

    pub fn complete(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: Anchor,
        cx: &App,
    ) -> Option<String> {
        if let Self::Spawned(client) = self {
            let buffer_id = buffer.entity_id();
            let buffer = buffer.read(cx);
            let path = buffer
                .file()
                .and_then(|file| Some(file.as_local()?.abs_path(cx)))
                .unwrap_or_else(|| PathBuf::from("untitled"))
                .to_string_lossy()
                .to_string();
            let content = buffer.text();
            let offset = cursor_position.to_offset(&buffer);
            let state_id = client.next_state_id;
            client.next_state_id.0 += 1;

            let (updates_tx, mut updates_rx) = watch::channel();
            postage::stream::Stream::try_recv(&mut updates_rx).unwrap();

            client.states.insert(
                state_id,
                AmazonQCompletionState {
                    buffer_id,
                    prefix_anchor: cursor_position,
                    prefix_offset: offset,
                    text: String::new(),
                    dedent: String::new(),
                    updates_tx,
                },
            );
            // ensure the states map is max 1000 elements
            if client.states.len() > 1000 {
                // state id is monotonic so it's sufficient to remove the first element
                client
                    .states
                    .remove(&client.states.keys().next().unwrap().clone());
            }

            client.request_completion(state_id, path, content, offset);

            Some(AmazonQCompletion {
                id: state_id,
                updates: updates_rx,
            })
        } else {
            None
        }
    }

    pub fn completion(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: Anchor,
        cx: &App,
    ) -> Option<&str> {
        if let Self::Spawned(client) = self {
            find_relevant_completion(
                &client.states,
                buffer.entity_id(),
                &buffer.read(cx).snapshot(),
                cursor_position,
            )
        } else {
            None
        }
    }

    pub fn sign_out(&mut self) {
        if let Self::Spawned(client) = self {
            client.sign_out();
        }
    }
}

pub struct AmazonQClient {
    aws_client: amzn_codewhisperer_client::Client,
    pub account_status: AccountStatus,
}

impl AmazonQClient {
    fn new(http_client: Arc<dyn HttpClient>, cx: &mut Context<AmazonQ>) -> Result<Self> {}

    fn sign_out(&mut self) {}
}
