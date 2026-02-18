use std::rc::Rc;

use gpui::display_tree::{
    DisplayColor, DisplayNode, DisplayNodeKind, DisplayTree, WireFrame,
};
use gpui::{
    div, px, AnyElement, Context, IntoElement, ParentElement, Render, Rgba, SharedString, Styled,
    Window,
};

use crate::Connection;

/// Root GPUI view for the browser client.
///
/// Holds the current DisplayTree received from the streaming server and renders
/// it as real GPUI elements. Each frame, the server sends a Snapshot or Delta;
/// `apply_frame()` updates the tree and `cx.notify()` triggers a re-render.
///
/// Every DisplayNode maps to a GPUI div positioned absolutely using the
/// server-computed bounds. This avoids needing a local text measurement system
/// for v1 -- the server already ran Taffy with real font metrics.
pub struct RemoteView {
    tree: Option<DisplayTree>,
    connection: Rc<Connection>,
}

impl RemoteView {
    pub fn new(connection: Rc<Connection>) -> Self {
        Self {
            tree: None,
            connection,
        }
    }

    /// Process an incoming WireFrame from the server.
    pub fn apply_frame(&mut self, frame: WireFrame) {
        match frame {
            WireFrame::Snapshot(tree) => {
                log::debug!(
                    "frame {} received ({}x{} viewport)",
                    tree.frame_id,
                    f32::from(tree.viewport.width),
                    f32::from(tree.viewport.height),
                );
                self.tree = Some(tree);
            }
            WireFrame::Delta(delta) => {
                log::debug!(
                    "delta for frame {} (base {}, {} patches)",
                    delta.frame_id,
                    delta.base_frame_id,
                    delta.patches.len(),
                );
                // Delta application deferred -- request a full snapshot instead.
                let request = WireFrame::RequestSnapshot;
                self.connection.send(&request).ok();
            }
            WireFrame::ActionAck {
                node_id,
                result_frame_id,
            } => {
                log::debug!(
                    "action ack: node {:?} -> frame {}",
                    node_id,
                    result_frame_id,
                );
            }
            WireFrame::SetViewport {
                width,
                height,
                scale_factor,
            } => {
                log::info!(
                    "server viewport: {}x{} @{}x",
                    width,
                    height,
                    scale_factor,
                );
            }
            WireFrame::Ping(seq) => {
                self.connection.send(&WireFrame::Pong(seq)).ok();
            }
            _ => {}
        }
    }

    /// Convert a DisplayNode subtree into GPUI elements.
    ///
    /// Each node becomes a div positioned using server-computed bounds.
    /// Children are nested inside their parent for correct paint order.
    fn render_node(&self, node: &DisplayNode) -> AnyElement {
        let mut el = div();

        if let Some(bounds) = &node.bounds {
            el = el
                .absolute()
                .left(px(f32::from(bounds.origin.x)))
                .top(px(f32::from(bounds.origin.y)))
                .w(px(f32::from(bounds.size.width)))
                .h(px(f32::from(bounds.size.height)));
        }

        if let Some(bg) = &node.style.visual.background {
            el = el.bg(to_rgba(bg));
        }

        if let Some(bc) = &node.style.visual.border_color {
            el = el.border_color(to_rgba(bc));
        }
        if let Some([top, right, bottom, left]) = node.style.visual.border_widths {
            if top > 0.0 || right > 0.0 || bottom > 0.0 || left > 0.0 {
                el = el.border_1();
            }
        }

        if let Some([tl, ..]) = node.style.visual.corner_radii {
            if tl > 0.0 {
                el = el.rounded(px(tl));
            }
        }

        if let Some(opacity) = node.style.visual.opacity {
            el = el.opacity(opacity);
        }

        if let Some(text) = &node.style.text {
            if let Some(color) = &text.color {
                el = el.text_color(to_rgba(color));
            }
            if let Some(size) = text.font_size {
                el = el.text_size(px(size));
            }
        }

        if !node.style.visual.visible {
            el = el.invisible();
        }

        match &node.kind {
            DisplayNodeKind::Container { .. } => {
                let children: Vec<AnyElement> = node
                    .children
                    .iter()
                    .map(|child| self.render_node(child))
                    .collect();
                el.children(children).into_any_element()
            }

            DisplayNodeKind::Text { content, .. }
            | DisplayNodeKind::InteractiveText { content, .. } => {
                el.child(SharedString::from(content.clone()))
                    .into_any_element()
            }

            DisplayNodeKind::UniformList { .. }
            | DisplayNodeKind::List { .. }
            | DisplayNodeKind::Anchored { .. } => {
                let children: Vec<AnyElement> = node
                    .children
                    .iter()
                    .map(|child| self.render_node(child))
                    .collect();
                el.children(children).into_any_element()
            }

            DisplayNodeKind::Svg { .. }
            | DisplayNodeKind::Image { .. }
            | DisplayNodeKind::Canvas { .. } => el.into_any_element(),
        }
    }
}

impl Render for RemoteView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let root = div().relative().size_full();

        if let Some(tree) = &self.tree {
            root.child(self.render_node(&tree.root))
        } else {
            root.child(
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(to_rgba(&DisplayColor {
                        r: 0.75,
                        g: 0.79,
                        b: 0.96,
                        a: 1.0,
                    }))
                    .child("Connecting to Zed server..."),
            )
        }
    }
}

fn to_rgba(c: &DisplayColor) -> Rgba {
    Rgba {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a,
    }
}
