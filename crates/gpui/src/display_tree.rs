//! Display tree: a serializable snapshot of GPUI's element tree.
//!
//! The display tree captures the structure, styles, text content, and interaction
//! capabilities of every element in a frame. It is produced on the server during
//! GPUI's normal render/layout/prepaint passes and shipped to a browser client,
//! which hydrates it into real GPUI elements for local layout, paint, and input.
//!
//! Gated behind the `headless-web` feature. When the feature is off, capture
//! points in the render pipeline compile to nothing.
//!
//! Wire format uses protobuf (prost) to match the rest of the Zed codebase.

use std::ops::Range;

// Re-export the prost-generated types.
#[allow(missing_docs)]
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/gpui.display_tree.rs"));
}

// Re-export commonly used proto types at module level for ergonomics.
pub use proto::display_action_kind;
pub use proto::display_image_source;
pub use proto::display_length;
pub use proto::display_node_kind;
pub use proto::display_tree_patch;
pub use proto::wire_frame;
pub use proto::ActionAck;
pub use proto::AnchoredKind;
pub use proto::Bounds;
pub use proto::ByteRange;
pub use proto::CanvasKind;
pub use proto::ClickAction;
pub use proto::ContainerKind;
pub use proto::DisplayAction;
pub use proto::DisplayActionKind;
pub use proto::DisplayBoxShadow;
pub use proto::DisplayColor;
pub use proto::DisplayImageSource;
pub use proto::DisplayLayoutStyle;
pub use proto::DisplayLength;
pub use proto::DisplayModifiers;
pub use proto::DisplayNode;
pub use proto::DisplayNodeId;
pub use proto::DisplayNodeKind;
pub use proto::DisplayStyle;
pub use proto::DisplayTextRun;
pub use proto::DisplayTextStyle;
pub use proto::DisplayTree;
pub use proto::DisplayTreeDelta;
pub use proto::DisplayTreePatch;
pub use proto::DisplayVisualStyle;
pub use proto::Empty;
pub use proto::HoverAction;
pub use proto::ImageKind;
pub use proto::InsertChildPatch;
pub use proto::InteractionFlags;
pub use proto::InteractiveTextKind;
pub use proto::KeyDownAction;
pub use proto::KeyUpAction;
pub use proto::ListKind;
pub use proto::MouseDownAction;
pub use proto::MouseMoveAction;
pub use proto::MouseUpAction;
pub use proto::Point;
pub use proto::RemoveChildPatch;
pub use proto::ReplaceNodePatch;
pub use proto::ResizeAction;
pub use proto::ScrollAction;
pub use proto::SetViewport;
pub use proto::Size;
pub use proto::SvgKind;
pub use proto::TextKind;
pub use proto::UniformListKind;
pub use proto::UpdateBoundsPatch;
pub use proto::UpdateListRangePatch;
pub use proto::UpdateScrollOffsetPatch;
pub use proto::UpdateStylePatch;
pub use proto::UpdateTextPatch;
pub use proto::ViewportChanged;
pub use proto::WireFrame;

// ---------------------------------------------------------------------------
// InteractionFlags constants
// ---------------------------------------------------------------------------

impl InteractionFlags {
    pub const NONE: u32 = 0;
    pub const CLICKABLE: u32 = 1 << 0;
    pub const HOVERABLE: u32 = 1 << 1;
    pub const SCROLLABLE: u32 = 1 << 2;
    pub const FOCUSABLE: u32 = 1 << 3;
    pub const KEY_INPUT: u32 = 1 << 4;
    pub const MOUSE_DOWN: u32 = 1 << 5;
    pub const MOUSE_UP: u32 = 1 << 6;
    pub const MOUSE_MOVE: u32 = 1 << 7;
    pub const DRAGGABLE: u32 = 1 << 8;
    pub const DROPPABLE: u32 = 1 << 9;
    pub const HAS_ACTIONS: u32 = 1 << 10;

    pub fn none() -> Self {
        Self { flags: Self::NONE }
    }

    pub fn contains_flag(&self, flag: u32) -> bool {
        (self.flags & flag) == flag
    }

    pub fn is_empty(&self) -> bool {
        self.flags == 0
    }

    pub fn from_interactivity(interactivity: &crate::Interactivity) -> Self {
        let mut flags = Self::NONE;

        if !interactivity.click_listeners.is_empty()
            || !interactivity.aux_click_listeners.is_empty()
        {
            flags |= Self::CLICKABLE;
        }
        if interactivity.hover_listener.is_some() || interactivity.hover_style.is_some() {
            flags |= Self::HOVERABLE;
        }
        if interactivity.scroll_offset.is_some() {
            flags |= Self::SCROLLABLE;
        }
        if interactivity.focusable || interactivity.tracked_focus_handle.is_some() {
            flags |= Self::FOCUSABLE;
        }
        if !interactivity.key_down_listeners.is_empty()
            || !interactivity.key_up_listeners.is_empty()
        {
            flags |= Self::KEY_INPUT;
        }
        if !interactivity.mouse_down_listeners.is_empty() {
            flags |= Self::MOUSE_DOWN;
        }
        if !interactivity.mouse_up_listeners.is_empty() {
            flags |= Self::MOUSE_UP;
        }
        if !interactivity.mouse_move_listeners.is_empty() {
            flags |= Self::MOUSE_MOVE;
        }
        if interactivity.drag_listener.is_some() {
            flags |= Self::DRAGGABLE;
        }
        if !interactivity.drop_listeners.is_empty() {
            flags |= Self::DROPPABLE;
        }
        if !interactivity.action_listeners.is_empty() {
            flags |= Self::HAS_ACTIONS;
        }

        Self { flags }
    }
}

// ---------------------------------------------------------------------------
// DisplayNodeId helpers
// ---------------------------------------------------------------------------

impl DisplayNodeId {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl std::hash::Hash for DisplayNodeId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for DisplayNodeId {}

impl Copy for DisplayNodeId {}

// ---------------------------------------------------------------------------
// DisplayColor conversion helpers
// ---------------------------------------------------------------------------

impl DisplayColor {
    pub fn from_hsla(hsla: crate::Hsla) -> Self {
        let rgba: crate::Rgba = hsla.into();
        Self {
            r: rgba.r,
            g: rgba.g,
            b: rgba.b,
            a: rgba.a,
        }
    }
}

// ---------------------------------------------------------------------------
// DisplayLength conversion helpers
// ---------------------------------------------------------------------------

impl DisplayLength {
    fn from_gpui_length(length: &crate::Length) -> Option<Self> {
        match length {
            crate::Length::Definite(d) => Self::from_gpui_definite_length_opt(d),
            crate::Length::Auto => Some(DisplayLength {
                length: Some(display_length::Length::Auto(true)),
            }),
        }
    }

    fn from_gpui_definite_length(d: &crate::DefiniteLength) -> Self {
        Self::from_gpui_definite_length_opt(d).unwrap_or(DisplayLength {
            length: Some(display_length::Length::Px(0.0)),
        })
    }

    fn from_gpui_definite_length_opt(d: &crate::DefiniteLength) -> Option<Self> {
        match d {
            crate::DefiniteLength::Absolute(crate::AbsoluteLength::Pixels(px)) => {
                Some(DisplayLength {
                    length: Some(display_length::Length::Px(px.0)),
                })
            }
            crate::DefiniteLength::Absolute(crate::AbsoluteLength::Rems(rems)) => {
                Some(DisplayLength {
                    length: Some(display_length::Length::Px(rems.0 * 16.0)),
                })
            }
            crate::DefiniteLength::Fraction(f) => Some(DisplayLength {
                length: Some(display_length::Length::Percent(*f * 100.0)),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// DisplayStyle conversion from GPUI Style
// ---------------------------------------------------------------------------

fn absolute_length_to_f32(len: &crate::AbsoluteLength) -> f32 {
    match len {
        crate::AbsoluteLength::Pixels(px) => px.0,
        crate::AbsoluteLength::Rems(rems) => rems.0 * 16.0,
    }
}

impl DisplayStyle {
    pub fn from_gpui_style(style: &crate::Style) -> Self {
        Self {
            display: Some(DisplayLayoutStyle::from_gpui_style(style)),
            visual: Some(DisplayVisualStyle::from_gpui_style(style)),
            text: style.text.color.map(|color| DisplayTextStyle {
                color: Some(DisplayColor::from_hsla(color)),
                font_family: None,
                font_size: None,
                font_weight: None,
                font_style: None,
                line_height: None,
            }),
        }
    }
}

impl DisplayLayoutStyle {
    fn from_gpui_style(style: &crate::Style) -> Self {
        Self {
            display: Some(format!("{:?}", style.display)),
            flex_direction: Some(format!("{:?}", style.flex_direction)),
            flex_grow: if style.flex_grow != 0.0 {
                Some(style.flex_grow)
            } else {
                None
            },
            flex_shrink: if style.flex_shrink != 1.0 {
                Some(style.flex_shrink)
            } else {
                None
            },
            width: DisplayLength::from_gpui_length(&style.size.width),
            height: DisplayLength::from_gpui_length(&style.size.height),
            min_width: DisplayLength::from_gpui_length(&style.min_size.width),
            max_width: DisplayLength::from_gpui_length(&style.max_size.width),
            min_height: DisplayLength::from_gpui_length(&style.min_size.height),
            max_height: DisplayLength::from_gpui_length(&style.max_size.height),
            padding: vec![
                DisplayLength::from_gpui_definite_length(&style.padding.top),
                DisplayLength::from_gpui_definite_length(&style.padding.right),
                DisplayLength::from_gpui_definite_length(&style.padding.bottom),
                DisplayLength::from_gpui_definite_length(&style.padding.left),
            ],
            margin: Vec::new(),
            gap: None,
            align_items: style.align_items.map(|v| format!("{:?}", v)),
            justify_content: style.justify_content.map(|v| format!("{:?}", v)),
            position: Some(format!("{:?}", style.position)),
            overflow_x: Some(format!("{:?}", style.overflow.x)),
            overflow_y: Some(format!("{:?}", style.overflow.y)),
        }
    }
}

impl DisplayVisualStyle {
    fn from_gpui_style(style: &crate::Style) -> Self {
        Self {
            background: style.background.as_ref().and_then(|fill| match fill {
                crate::Fill::Color(bg) => Some(DisplayColor::from_hsla(bg.solid)),
            }),
            border_color: style.border_color.map(DisplayColor::from_hsla),
            border_widths: vec![
                absolute_length_to_f32(&style.border_widths.top),
                absolute_length_to_f32(&style.border_widths.right),
                absolute_length_to_f32(&style.border_widths.bottom),
                absolute_length_to_f32(&style.border_widths.left),
            ],
            corner_radii: vec![
                absolute_length_to_f32(&style.corner_radii.top_left),
                absolute_length_to_f32(&style.corner_radii.top_right),
                absolute_length_to_f32(&style.corner_radii.bottom_right),
                absolute_length_to_f32(&style.corner_radii.bottom_left),
            ],
            box_shadows: Vec::new(),
            opacity: style.opacity.filter(|&o| o < 1.0),
            cursor: style.mouse_cursor.map(|c| format!("{:?}", c)),
            visible: style.visibility != crate::Visibility::Hidden,
        }
    }
}

// ---------------------------------------------------------------------------
// Builder for constructing a DisplayTree during GPUI's render pipeline
// ---------------------------------------------------------------------------

pub struct DisplayTreeBuilder {
    frame_id: u64,
    viewport: crate::Size<crate::Pixels>,
    node_stack: Vec<DisplayNode>,
    next_id: u64,
}

impl DisplayTreeBuilder {
    pub fn new(frame_id: u64, viewport: crate::Size<crate::Pixels>) -> Self {
        Self {
            frame_id,
            viewport,
            node_stack: Vec::with_capacity(64),
            next_id: 0,
        }
    }

    fn next_node_id(&mut self) -> DisplayNodeId {
        let id = self.next_id;
        self.next_id += 1;
        DisplayNodeId { id }
    }

    pub fn push_container(
        &mut self,
        element_id: Option<String>,
        style: DisplayStyle,
        interactions: InteractionFlags,
        scroll_offset: Option<crate::Point<crate::Pixels>>,
        group: Option<String>,
    ) {
        let node_id = self.next_node_id();
        let node = DisplayNode {
            id: Some(node_id),
            element_id,
            kind: Some(DisplayNodeKind {
                kind: Some(display_node_kind::Kind::Container(ContainerKind {
                    scroll_offset: scroll_offset.map(|p| Point { x: p.x.0, y: p.y.0 }),
                    group,
                })),
            }),
            style: Some(style),
            bounds: None,
            content_size: None,
            interactions: Some(interactions),
            children: Vec::new(),
        };
        self.node_stack.push(node);
    }

    pub fn push_text(
        &mut self,
        element_id: Option<String>,
        style: DisplayStyle,
        content: String,
        runs: Vec<DisplayTextRun>,
    ) {
        let node_id = self.next_node_id();
        let node = DisplayNode {
            id: Some(node_id),
            element_id,
            kind: Some(DisplayNodeKind {
                kind: Some(display_node_kind::Kind::Text(TextKind { content, runs })),
            }),
            style: Some(style),
            bounds: None,
            content_size: None,
            interactions: Some(InteractionFlags::none()),
            children: Vec::new(),
        };
        self.add_leaf(node);
    }

    pub fn push_image(
        &mut self,
        element_id: Option<String>,
        style: DisplayStyle,
        source: DisplayImageSource,
        object_fit: Option<String>,
        grayscale: bool,
    ) {
        let node_id = self.next_node_id();
        let node = DisplayNode {
            id: Some(node_id),
            element_id,
            kind: Some(DisplayNodeKind {
                kind: Some(display_node_kind::Kind::Image(ImageKind {
                    source: Some(source),
                    object_fit,
                    grayscale,
                })),
            }),
            style: Some(style),
            bounds: None,
            content_size: None,
            interactions: Some(InteractionFlags::none()),
            children: Vec::new(),
        };
        self.add_leaf(node);
    }

    pub fn push_svg(
        &mut self,
        element_id: Option<String>,
        style: DisplayStyle,
        path: String,
        color: Option<DisplayColor>,
    ) {
        let node_id = self.next_node_id();
        let node = DisplayNode {
            id: Some(node_id),
            element_id,
            kind: Some(DisplayNodeKind {
                kind: Some(display_node_kind::Kind::Svg(SvgKind { path, color })),
            }),
            style: Some(style),
            bounds: None,
            content_size: None,
            interactions: Some(InteractionFlags::none()),
            children: Vec::new(),
        };
        self.add_leaf(node);
    }

    pub fn push_uniform_list(
        &mut self,
        element_id: Option<String>,
        style: DisplayStyle,
        interactions: InteractionFlags,
        total_items: usize,
        item_height: f32,
        visible_range: Range<usize>,
        scroll_offset: f32,
    ) {
        let node_id = self.next_node_id();
        let node = DisplayNode {
            id: Some(node_id),
            element_id,
            kind: Some(DisplayNodeKind {
                kind: Some(display_node_kind::Kind::UniformList(UniformListKind {
                    total_items: total_items as u64,
                    item_height,
                    visible_range_start: visible_range.start as u64,
                    visible_range_end: visible_range.end as u64,
                    scroll_offset,
                })),
            }),
            style: Some(style),
            bounds: None,
            content_size: None,
            interactions: Some(interactions),
            children: Vec::new(),
        };
        self.node_stack.push(node);
    }

    pub fn set_current_bounds(&mut self, bounds: crate::Bounds<crate::Pixels>) {
        if let Some(node) = self.node_stack.last_mut() {
            node.bounds = Some(Bounds {
                origin: Some(Point {
                    x: bounds.origin.x.0,
                    y: bounds.origin.y.0,
                }),
                size: Some(Size {
                    width: bounds.size.width.0,
                    height: bounds.size.height.0,
                }),
            });
        }
    }

    pub fn set_current_content_size(&mut self, size: crate::Size<crate::Pixels>) {
        if let Some(node) = self.node_stack.last_mut() {
            node.content_size = Some(Size {
                width: size.width.0,
                height: size.height.0,
            });
        }
    }

    pub fn set_current_kind(&mut self, kind: DisplayNodeKind) {
        if let Some(node) = self.node_stack.last_mut() {
            node.kind = Some(kind);
        }
    }

    pub fn pop_node(&mut self) {
        if self.node_stack.len() > 1 {
            if let Some(node) = self.node_stack.pop() {
                if let Some(parent) = self.node_stack.last_mut() {
                    parent.children.push(node);
                }
            }
        }
    }

    pub fn finish(mut self) -> Option<DisplayTree> {
        while self.node_stack.len() > 1 {
            self.pop_node();
        }
        self.node_stack.pop().map(|root| DisplayTree {
            frame_id: self.frame_id,
            viewport: Some(Size {
                width: self.viewport.width.0,
                height: self.viewport.height.0,
            }),
            root: Some(root),
        })
    }

    fn add_leaf(&mut self, node: DisplayNode) {
        if let Some(parent) = self.node_stack.last_mut() {
            parent.children.push(node);
        }
    }

    pub fn push_shaped_text(
        &mut self,
        content: String,
        bounds: crate::Bounds<crate::Pixels>,
        runs: Vec<DisplayTextRun>,
    ) {
        let node_id = self.next_node_id();
        let node = DisplayNode {
            id: Some(node_id),
            element_id: None,
            kind: Some(DisplayNodeKind {
                kind: Some(display_node_kind::Kind::Text(TextKind { content, runs })),
            }),
            style: Some(DisplayStyle::default()),
            bounds: Some(Bounds {
                origin: Some(Point {
                    x: bounds.origin.x.0,
                    y: bounds.origin.y.0,
                }),
                size: Some(Size {
                    width: bounds.size.width.0,
                    height: bounds.size.height.0,
                }),
            }),
            content_size: None,
            interactions: Some(InteractionFlags::none()),
            children: Vec::new(),
        };
        self.add_leaf(node);
    }
}

// ---------------------------------------------------------------------------
// Delta diffing
// ---------------------------------------------------------------------------

fn node_id(node: &DisplayNode) -> u64 {
    node.id.as_ref().map_or(0, |id| id.id)
}

pub fn diff_display_trees(old: &DisplayTree, new: &DisplayTree) -> Option<DisplayTreeDelta> {
    let (Some(old_root), Some(new_root)) = (&old.root, &new.root) else {
        return None;
    };
    let mut patches = Vec::new();
    diff_nodes(old_root, new_root, &mut patches);
    if patches.is_empty() {
        None
    } else {
        Some(DisplayTreeDelta {
            frame_id: new.frame_id,
            base_frame_id: old.frame_id,
            patches,
        })
    }
}

fn diff_nodes(old: &DisplayNode, new: &DisplayNode, patches: &mut Vec<DisplayTreePatch>) {
    if node_id(old) != node_id(new)
        || std::mem::discriminant(&old.kind) != std::mem::discriminant(&new.kind)
    {
        patches.push(DisplayTreePatch {
            patch: Some(display_tree_patch::Patch::ReplaceNode(ReplaceNodePatch {
                target: old.id.clone(),
                node: Some(new.clone()),
            })),
        });
        return;
    }

    // Check for scroll offset changes on containers.
    if let (
        Some(DisplayNodeKind {
            kind: Some(display_node_kind::Kind::Container(old_container)),
        }),
        Some(DisplayNodeKind {
            kind: Some(display_node_kind::Kind::Container(new_container)),
        }),
    ) = (&old.kind, &new.kind)
    {
        if old_container.scroll_offset != new_container.scroll_offset {
            if let Some(offset) = &new_container.scroll_offset {
                patches.push(DisplayTreePatch {
                    patch: Some(display_tree_patch::Patch::UpdateScrollOffset(
                        UpdateScrollOffsetPatch {
                            target: new.id.clone(),
                            offset: Some(offset.clone()),
                        },
                    )),
                });
            }
        }
    }

    // Check for text content changes.
    if let (
        Some(DisplayNodeKind {
            kind: Some(display_node_kind::Kind::Text(old_text)),
        }),
        Some(DisplayNodeKind {
            kind: Some(display_node_kind::Kind::Text(new_text)),
        }),
    ) = (&old.kind, &new.kind)
    {
        if old_text.content != new_text.content || old_text.runs.len() != new_text.runs.len() {
            patches.push(DisplayTreePatch {
                patch: Some(display_tree_patch::Patch::UpdateText(UpdateTextPatch {
                    target: new.id.clone(),
                    content: new_text.content.clone(),
                    runs: new_text.runs.clone(),
                })),
            });
        }
    }

    // Check for bounds changes.
    if old.bounds != new.bounds {
        if let Some(bounds) = &new.bounds {
            patches.push(DisplayTreePatch {
                patch: Some(display_tree_patch::Patch::UpdateBounds(UpdateBoundsPatch {
                    target: new.id.clone(),
                    bounds: Some(bounds.clone()),
                })),
            });
        }
    }

    // Diff children.
    let old_len = old.children.len();
    let new_len = new.children.len();
    let common = old_len.min(new_len);

    for i in 0..common {
        diff_nodes(&old.children[i], &new.children[i], patches);
    }

    for i in common..new_len {
        patches.push(DisplayTreePatch {
            patch: Some(display_tree_patch::Patch::InsertChild(InsertChildPatch {
                parent: new.id.clone(),
                index: i as u64,
                node: Some(new.children[i].clone()),
            })),
        });
    }

    for i in (common..old_len).rev() {
        patches.push(DisplayTreePatch {
            patch: Some(display_tree_patch::Patch::RemoveChild(RemoveChildPatch {
                parent: old.id.clone(),
                index: i as u64,
            })),
        });
    }
}

// ---------------------------------------------------------------------------
// Wire protocol serialization via prost
// ---------------------------------------------------------------------------

impl WireFrame {
    pub fn serialize(&self) -> Result<Vec<u8>, prost::EncodeError> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        Ok(buf)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        use prost::Message;
        Self::decode(bytes)
    }
}

impl DisplayTree {
    pub fn serialize(&self) -> Result<Vec<u8>, prost::EncodeError> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        Ok(buf)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        use prost::Message;
        Self::decode(bytes)
    }
}

impl DisplayTreeDelta {
    pub fn serialize(&self) -> Result<Vec<u8>, prost::EncodeError> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        Ok(buf)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        use prost::Message;
        Self::decode(bytes)
    }
}
