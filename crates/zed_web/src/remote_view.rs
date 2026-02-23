use std::rc::Rc;

use gpui::display_tree::{
    display_node_kind, wire_frame, DisplayColor, DisplayModifiers, DisplayNode, DisplayTree,
    InteractionFlags, WireFrame,
};
use gpui::{
    div, px, AnyElement, Context, Div, InteractiveElement, IntoElement, ParentElement, Render,
    Rgba, SharedString, Styled, Window,
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
/// for v1 — the server already ran Taffy with real font metrics.
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
        let Some(inner) = frame.frame else { return };
        match inner {
            wire_frame::Frame::Snapshot(tree) => {
                if let Some(viewport) = &tree.viewport {
                    log::debug!(
                        "frame {} received ({}x{} viewport)",
                        tree.frame_id,
                        viewport.width,
                        viewport.height,
                    );
                }
                self.tree = Some(tree);
            }
            wire_frame::Frame::Delta(delta) => {
                log::debug!(
                    "delta for frame {} (base {}, {} patches)",
                    delta.frame_id,
                    delta.base_frame_id,
                    delta.patches.len(),
                );
                // We don't apply deltas yet — request a full snapshot instead.
                self.connection.request_snapshot().ok();
            }
            wire_frame::Frame::ActionAck(ack) => {
                log::debug!(
                    "action ack: node {:?} -> frame {}",
                    ack.node_id,
                    ack.result_frame_id,
                );
            }
            wire_frame::Frame::SetViewport(sv) => {
                log::info!(
                    "server viewport: {}x{} @{}x",
                    sv.width,
                    sv.height,
                    sv.scale_factor,
                );
            }
            wire_frame::Frame::Ping(seq) => {
                self.connection.send_pong(seq).ok();
            }
            _ => {}
        }
    }

    /// Convert a DisplayNode subtree into GPUI elements, attaching interaction
    /// handlers for nodes that have the appropriate InteractionFlags set.
    ///
    /// `parent_origin` is the window-absolute origin of the parent node.
    /// Since the server captures bounds in window-absolute coordinates but
    /// CSS absolute positioning is relative to the positioned parent, we
    /// subtract the parent origin to get parent-relative coordinates.
    fn render_node(&self, node: &DisplayNode, parent_origin: (f32, f32)) -> AnyElement {
        let mut el = div();

        let node_origin = node
            .bounds
            .as_ref()
            .and_then(|b| b.origin.as_ref())
            .map(|o| (o.x, o.y))
            .unwrap_or(parent_origin);

        if let Some(bounds) = &node.bounds {
            if let (Some(origin), Some(size)) = (&bounds.origin, &bounds.size) {
                el = el
                    .absolute()
                    .left(px(origin.x - parent_origin.0))
                    .top(px(origin.y - parent_origin.1))
                    .w(px(size.width))
                    .h(px(size.height));
            }
        }

        el = apply_visual_style(el, node);

        let flags = node.interactions.as_ref().map(|i| i.flags).unwrap_or(0);
        let node_id = node.id.as_ref().map(|id| id.id).unwrap_or(0);
        let element_id = node.element_id.clone();

        // Mouse button events (click, mousedown, mouseup) are handled by
        // window-level DOM listeners in lib.rs — GPUI does its own hit-testing
        // so it needs raw window-position events for all clicks, not just
        // nodes with CLICKABLE flags.

        // Attach scroll handler if the node is scrollable.
        if flags & InteractionFlags::SCROLLABLE != 0 {
            let conn = self.connection.clone();
            let eid = element_id.clone();
            el = el.on_scroll_wheel(move |event, _window, _cx| {
                let delta = event.delta.pixel_delta(px(1.0));
                let dx: f32 = delta.x.into();
                let dy: f32 = delta.y.into();
                conn.send_scroll(node_id, eid.clone(), dx, dy, DisplayModifiers::default())
                    .ok();
            });
        }

        let kind = node.kind.as_ref().and_then(|k| k.kind.as_ref());

        match kind {
            Some(display_node_kind::Kind::Container(_)) => {
                let children: Vec<AnyElement> = node
                    .children
                    .iter()
                    .map(|child| self.render_node(child, node_origin))
                    .collect();
                el.children(children).into_any_element()
            }

            Some(display_node_kind::Kind::Text(text)) => el
                .child(SharedString::from(text.content.clone()))
                .into_any_element(),

            Some(display_node_kind::Kind::InteractiveText(text)) => el
                .child(SharedString::from(text.content.clone()))
                .into_any_element(),

            Some(display_node_kind::Kind::UniformList(_))
            | Some(display_node_kind::Kind::List(_))
            | Some(display_node_kind::Kind::Anchored(_)) => {
                let children: Vec<AnyElement> = node
                    .children
                    .iter()
                    .map(|child| self.render_node(child, node_origin))
                    .collect();
                el.children(children).into_any_element()
            }

            Some(display_node_kind::Kind::Svg(_))
            | Some(display_node_kind::Kind::Image(_))
            | Some(display_node_kind::Kind::Canvas(_))
            | None => el.into_any_element(),
        }
    }
}

/// Apply visual styles from a DisplayNode to a div element.
fn apply_visual_style(mut el: Div, node: &DisplayNode) -> Div {
    let Some(style) = &node.style else {
        return el;
    };

    if let Some(visual) = &style.visual {
        if let Some(bg) = &visual.background {
            el = el.bg(to_rgba(bg));
        }

        if let Some(bc) = &visual.border_color {
            el = el.border_color(to_rgba(bc));
        }
        if visual.border_widths.len() >= 4 {
            let top = visual.border_widths[0];
            let right = visual.border_widths[1];
            let bottom = visual.border_widths[2];
            let left = visual.border_widths[3];
            if top > 0.0 || right > 0.0 || bottom > 0.0 || left > 0.0 {
                el = el.border_1();
            }
        }

        if !visual.corner_radii.is_empty() && visual.corner_radii[0] > 0.0 {
            el = el.rounded(px(visual.corner_radii[0]));
        }

        if let Some(opacity) = visual.opacity {
            el = el.opacity(opacity);
        }

        if !visual.visible {
            el = el.invisible();
        }
    }

    if let Some(text) = &style.text {
        if let Some(color) = &text.color {
            el = el.text_color(to_rgba(color));
        }
        if let Some(size) = text.font_size {
            el = el.text_size(px(size));
        }
    }

    el
}

impl Render for RemoteView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let root = div().relative().size_full();

        if let Some(tree) = &self.tree {
            if let Some(root_node) = &tree.root {
                root.child(self.render_node(root_node, (0.0, 0.0)))
            } else {
                root
            }
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
