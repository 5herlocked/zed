use std::sync::Arc;

use gpui::{actions, App, Entity, EventEmitter, FocusHandle, Focusable};
use ui::prelude::*;
use workspace::item::{Item, ItemEvent};
use workspace::{AppState, Workspace};

pub fn init(app_state: &Arc<AppState>, cx: &mut App) {
    todo!()
}

pub struct AwsAuthPage {
    focus_handle: FocusHandle,
}

impl AwsAuthPage {
    pub fn new(_workspace: &Workspace, cx: &mut Context<Workspace>) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
        })
    }
}

impl EventEmitter<ItemEvent> for AwsAuthPage {}

impl Focusable for AwsAuthPage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for AwsAuthPage {
    type Event = ItemEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Settings))
    }

    fn tab_content_text(&self, _window: &Window, _cx: &App) -> Option<SharedString> {
        Some("AWS Authentication".into())
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }
}

impl Render for AwsAuthPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        todo!()
    }
}
