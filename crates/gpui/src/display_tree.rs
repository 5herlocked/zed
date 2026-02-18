//! Display tree: a serializable snapshot of GPUI's element tree.
//!
//! The display tree captures the structure, styles, text content, and interaction
//! capabilities of every element in a frame. It is produced on the server during
//! GPUI's normal render/layout/prepaint passes and shipped to a browser client,
//! which hydrates it into real GPUI elements for local layout, paint, and input.
//!
//! Gated behind the `headless-web` feature. When the feature is off, capture
//! points in the render pipeline compile to nothing.

use crate::{Bounds, Pixels, Point, Size};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use crate::{Hsla, Rgba};
use std::ops::Range;

/// Unique identifier for a node in the display tree.
///
/// Derived from the element's `GlobalElementId` when available, or assigned
/// sequentially during capture. Stable across frames for the same element,
/// enabling delta encoding and action forwarding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DisplayNodeId(pub u64);

/// Flags indicating which interaction types an element supports.
///
/// The browser uses these to decide which DOM events to capture and forward
/// to the server. The server maintains a mapping from `DisplayNodeId` to the
/// real closures registered during render.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionFlags(u32);

impl InteractionFlags {
    /// No interactions.
    pub const NONE: Self = Self(0);
    /// Element has click handlers.
    pub const CLICKABLE: Self = Self(1 << 0);
    /// Element has hover handlers.
    pub const HOVERABLE: Self = Self(1 << 1);
    /// Element is scrollable.
    pub const SCROLLABLE: Self = Self(1 << 2);
    /// Element can receive focus.
    pub const FOCUSABLE: Self = Self(1 << 3);
    /// Element handles key input.
    pub const KEY_INPUT: Self = Self(1 << 4);
    /// Element handles mouse down.
    pub const MOUSE_DOWN: Self = Self(1 << 5);
    /// Element handles mouse up.
    pub const MOUSE_UP: Self = Self(1 << 6);
    /// Element handles mouse move.
    pub const MOUSE_MOVE: Self = Self(1 << 7);
    /// Element is draggable.
    pub const DRAGGABLE: Self = Self(1 << 8);
    /// Element accepts drops.
    pub const DROPPABLE: Self = Self(1 << 9);
    /// Element has action handlers (typed GPUI actions).
    pub const HAS_ACTIONS: Self = Self(1 << 10);

    /// Combine two sets of flags.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Check whether a specific flag is set.
    pub const fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }

    /// Check whether any flags are set.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// A single node in the display tree, representing one GPUI element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayNode {
    /// Stable identifier for this node.
    pub id: DisplayNodeId,

    /// The GPUI `GlobalElementId` serialized as a dot-separated path string.
    /// Present only for elements that have an explicit element ID in GPUI.
    /// Used by the browser to forward interactions back to the server.
    pub element_id: Option<String>,

    /// Element type and type-specific payload.
    pub kind: DisplayNodeKind,

    /// Resolved style. Uses GPUI's `StyleRefinement` which already implements
    /// `Serialize` and `Deserialize` via the Refineable macro.
    pub style: DisplayStyle,

    /// Computed bounds from the layout pass. `None` until prepaint capture
    /// fills it in. The browser uses these as a fallback for hit testing
    /// when it hasn't performed its own layout yet (first frame).
    pub bounds: Option<Bounds<Pixels>>,

    /// Content size (may differ from bounds for scrollable elements).
    pub content_size: Option<Size<Pixels>>,

    /// Interaction capabilities. The browser captures and forwards only the
    /// event types flagged here.
    pub interactions: InteractionFlags,

    /// Child nodes, in render order.
    pub children: Vec<DisplayNode>,
}

/// Subset of style data carried by each display node.
///
/// This is a flattened representation rather than the full `StyleRefinement`
/// to keep serialization compact and avoid leaking GPUI's internal style
/// cascade into the wire format. Fields are resolved (not `Option`-wrapped
/// refinement deltas) because the server has already performed the cascade.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisplayStyle {
    /// Layout properties.
    pub display: DisplayLayoutStyle,
    /// Visual properties.
    pub visual: DisplayVisualStyle,
    /// Text properties (inherited down the tree, present when overridden).
    pub text: Option<DisplayTextStyle>,
}

/// Layout-related style properties.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisplayLayoutStyle {
    /// CSS display value.
    pub display: Option<String>,
    /// Flex direction.
    pub flex_direction: Option<String>,
    /// Flex grow factor.
    pub flex_grow: Option<f32>,
    /// Flex shrink factor.
    pub flex_shrink: Option<f32>,
    /// Width (fixed pixels or auto/percentage).
    pub width: Option<DisplayLength>,
    /// Height.
    pub height: Option<DisplayLength>,
    /// Min width.
    pub min_width: Option<DisplayLength>,
    /// Max width.
    pub max_width: Option<DisplayLength>,
    /// Min height.
    pub min_height: Option<DisplayLength>,
    /// Max height.
    pub max_height: Option<DisplayLength>,
    /// Padding (top, right, bottom, left).
    pub padding: Option<[DisplayLength; 4]>,
    /// Margin.
    pub margin: Option<[DisplayLength; 4]>,
    /// Gap between flex children.
    pub gap: Option<DisplayLength>,
    /// Align items (cross axis).
    pub align_items: Option<String>,
    /// Justify content (main axis).
    pub justify_content: Option<String>,
    /// Position (relative, absolute).
    pub position: Option<String>,
    /// Overflow behavior.
    pub overflow_x: Option<String>,
    /// Overflow behavior.
    pub overflow_y: Option<String>,
}

/// A length value that can be pixels, percentage, or auto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayLength {
    /// Fixed pixel value.
    Px(f32),
    /// Percentage of parent.
    Percent(f32),
    /// Auto sizing.
    Auto,
}

/// Visual style properties.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisplayVisualStyle {
    /// Background color as RGBA.
    pub background: Option<DisplayColor>,
    /// Border color.
    pub border_color: Option<DisplayColor>,
    /// Border widths (top, right, bottom, left).
    pub border_widths: Option<[f32; 4]>,
    /// Corner radii (top-left, top-right, bottom-right, bottom-left).
    pub corner_radii: Option<[f32; 4]>,
    /// Box shadows.
    pub box_shadows: Vec<DisplayBoxShadow>,
    /// Opacity (0.0 to 1.0).
    pub opacity: Option<f32>,
    /// Mouse cursor style.
    pub cursor: Option<String>,
    /// Visibility.
    pub visible: bool,
}

/// RGBA color for wire transport.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DisplayColor {
    /// Red channel (0.0 to 1.0).
    pub r: f32,
    /// Green channel.
    pub g: f32,
    /// Blue channel.
    pub b: f32,
    /// Alpha channel.
    pub a: f32,
}

/// Box shadow for wire transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayBoxShadow {
    /// Horizontal offset in pixels.
    pub offset_x: f32,
    /// Vertical offset in pixels.
    pub offset_y: f32,
    /// Blur radius in pixels.
    pub blur: f32,
    /// Spread radius in pixels.
    pub spread: f32,
    /// Shadow color.
    pub color: DisplayColor,
}

/// Text-specific style properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayTextStyle {
    /// Font family name.
    pub font_family: Option<String>,
    /// Font size in pixels.
    pub font_size: Option<f32>,
    /// Font weight (100-900).
    pub font_weight: Option<u16>,
    /// Italic.
    pub font_style: Option<String>,
    /// Text color.
    pub color: Option<DisplayColor>,
    /// Line height in pixels.
    pub line_height: Option<f32>,
}

/// Element-type-specific payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayNodeKind {
    /// A container element (GPUI `Div`).
    Container {
        /// Current scroll offset for scrollable containers.
        scroll_offset: Option<Point<Pixels>>,
        /// Group name for group-based hover/active styling.
        group: Option<String>,
    },

    /// A text element with styled runs.
    Text {
        /// The text content.
        content: String,
        /// Style runs: each run styles a substring.
        runs: Vec<DisplayTextRun>,
    },

    /// Text with interactive (clickable) regions.
    InteractiveText {
        /// The text content.
        content: String,
        /// Style runs.
        runs: Vec<DisplayTextRun>,
        /// Clickable regions as byte ranges.
        clickable_ranges: Vec<Range<usize>>,
    },

    /// An image element.
    Image {
        /// How to obtain the image data.
        source: DisplayImageSource,
        /// Object-fit behavior.
        object_fit: Option<String>,
        /// Whether grayscale filter is applied.
        grayscale: bool,
    },

    /// An SVG element.
    Svg {
        /// SVG content or path identifier.
        path: String,
        /// Fill color override.
        color: Option<DisplayColor>,
    },

    /// A virtualized uniform-height list (file trees, editor lines).
    UniformList {
        /// Total number of items in the list.
        total_items: usize,
        /// Height of each item in pixels.
        item_height: f32,
        /// Currently visible range of items (indices).
        visible_range: Range<usize>,
        /// Current scroll offset in pixels.
        scroll_offset: f32,
    },

    /// A virtualized variable-height list.
    List {
        /// Total number of items.
        total_items: usize,
        /// Currently visible range.
        visible_range: Range<usize>,
        /// Current scroll offset in pixels.
        scroll_offset: f32,
    },

    /// A positioned/anchored overlay element.
    Anchored {
        /// Which corner of the anchor element this is positioned relative to.
        anchor_corner: String,
    },

    /// A canvas element with custom paint logic.
    /// Since the paint closures can't be serialized, the server rasterizes
    /// the canvas content to an image and sends it as pixel data.
    Canvas {
        /// Server-rasterized content, if available.
        rasterized: Option<Vec<u8>>,
        /// Width of the rasterized image.
        rasterized_width: u32,
        /// Height of the rasterized image.
        rasterized_height: u32,
    },
}

/// A style run within a text element: styles a contiguous range of characters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayTextRun {
    /// Number of UTF-8 bytes this run covers.
    pub len: usize,
    /// Text color for this run.
    pub color: Option<DisplayColor>,
    /// Font weight for this run.
    pub font_weight: Option<u16>,
    /// Whether this run is italic.
    pub italic: bool,
    /// Whether this run has an underline.
    pub underline: bool,
    /// Whether this run has a strikethrough.
    pub strikethrough: bool,
    /// Background highlight color.
    pub background: Option<DisplayColor>,
}

/// Image source reference for wire transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayImageSource {
    /// URI to fetch the image from.
    Uri(String),
    /// Inline image data (PNG/JPEG bytes).
    Data(Vec<u8>),
}

/// A complete display tree snapshot for one frame.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayTree {
    /// Monotonically increasing frame counter.
    pub frame_id: u64,
    /// Viewport dimensions.
    pub viewport: Size<Pixels>,
    /// Root node of the element tree.
    pub root: DisplayNode,
}

/// A delta update between two display tree frames.
///
/// Rather than sending the full tree every frame, the server diffs against
/// the previous frame and sends only the changes. The browser applies
/// patches to its cached tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayTreeDelta {
    /// Frame ID this delta produces.
    pub frame_id: u64,
    /// Frame ID this delta is relative to.
    pub base_frame_id: u64,
    /// Ordered list of patches to apply.
    pub patches: Vec<DisplayTreePatch>,
}

/// A single patch operation in a display tree delta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayTreePatch {
    /// Replace an entire subtree rooted at the given node.
    ReplaceNode {
        /// ID of the node to replace.
        target: DisplayNodeId,
        /// The replacement subtree.
        node: DisplayNode,
    },

    /// Update only the style of a node.
    UpdateStyle {
        /// Target node.
        target: DisplayNodeId,
        /// New style.
        style: DisplayStyle,
    },

    /// Update the text content of a text node.
    UpdateText {
        /// Target node.
        target: DisplayNodeId,
        /// New text content.
        content: String,
        /// New style runs.
        runs: Vec<DisplayTextRun>,
    },

    /// Update the computed bounds of a node.
    UpdateBounds {
        /// Target node.
        target: DisplayNodeId,
        /// New bounds.
        bounds: Bounds<Pixels>,
    },

    /// Update the scroll offset of a scrollable node.
    UpdateScrollOffset {
        /// Target node.
        target: DisplayNodeId,
        /// New scroll offset.
        offset: Point<Pixels>,
    },

    /// Insert a child node at a specific index.
    InsertChild {
        /// Parent node.
        parent: DisplayNodeId,
        /// Index to insert at.
        index: usize,
        /// The child subtree to insert.
        node: DisplayNode,
    },

    /// Remove a child node at a specific index.
    RemoveChild {
        /// Parent node.
        parent: DisplayNodeId,
        /// Index of the child to remove.
        index: usize,
    },

    /// Update the visible range of a virtualized list.
    UpdateListRange {
        /// Target list node.
        target: DisplayNodeId,
        /// New visible range.
        visible_range: Range<usize>,
        /// New scroll offset.
        scroll_offset: f32,
        /// Replacement children for the new visible range.
        children: Vec<DisplayNode>,
    },
}

/// Builder for constructing a `DisplayTree` during GPUI's render pipeline.
///
/// Held on the `Window` when `headless-web` is enabled. Each render walk
/// pushes and pops nodes as elements are processed, building the tree
/// incrementally without an extra traversal.
pub struct DisplayTreeBuilder {
    frame_id: u64,
    viewport: Size<Pixels>,
    node_stack: Vec<DisplayNode>,
    next_id: u64,
}

impl DisplayTreeBuilder {
    /// Create a new builder for a frame.
    pub fn new(frame_id: u64, viewport: Size<Pixels>) -> Self {
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
        DisplayNodeId(id)
    }

    /// Push a new container node onto the stack. Subsequent pushes become
    /// children until `pop_node` is called.
    pub fn push_container(
        &mut self,
        element_id: Option<String>,
        style: DisplayStyle,
        interactions: InteractionFlags,
        scroll_offset: Option<Point<Pixels>>,
        group: Option<String>,
    ) {
        let node = DisplayNode {
            id: self.next_node_id(),
            element_id,
            kind: DisplayNodeKind::Container {
                scroll_offset,
                group,
            },
            style,
            bounds: None,
            content_size: None,
            interactions,
            children: Vec::new(),
        };
        self.node_stack.push(node);
    }

    /// Push a text node as a child of the current container.
    pub fn push_text(
        &mut self,
        element_id: Option<String>,
        style: DisplayStyle,
        content: String,
        runs: Vec<DisplayTextRun>,
    ) {
        let node = DisplayNode {
            id: self.next_node_id(),
            element_id,
            kind: DisplayNodeKind::Text { content, runs },
            style,
            bounds: None,
            content_size: None,
            interactions: InteractionFlags::NONE,
            children: Vec::new(),
        };
        self.add_leaf(node);
    }

    /// Push an image node as a child of the current container.
    pub fn push_image(
        &mut self,
        element_id: Option<String>,
        style: DisplayStyle,
        source: DisplayImageSource,
        object_fit: Option<String>,
        grayscale: bool,
    ) {
        let node = DisplayNode {
            id: self.next_node_id(),
            element_id,
            kind: DisplayNodeKind::Image {
                source,
                object_fit,
                grayscale,
            },
            style,
            bounds: None,
            content_size: None,
            interactions: InteractionFlags::NONE,
            children: Vec::new(),
        };
        self.add_leaf(node);
    }

    /// Push an SVG node as a child of the current container.
    pub fn push_svg(
        &mut self,
        element_id: Option<String>,
        style: DisplayStyle,
        path: String,
        color: Option<DisplayColor>,
    ) {
        let node = DisplayNode {
            id: self.next_node_id(),
            element_id,
            kind: DisplayNodeKind::Svg { path, color },
            style,
            bounds: None,
            content_size: None,
            interactions: InteractionFlags::NONE,
            children: Vec::new(),
        };
        self.add_leaf(node);
    }

    /// Push a uniform list node. Children are added between this call and
    /// the corresponding `pop_node`.
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
        let node = DisplayNode {
            id: self.next_node_id(),
            element_id,
            kind: DisplayNodeKind::UniformList {
                total_items,
                item_height,
                visible_range,
                scroll_offset,
            },
            style,
            bounds: None,
            content_size: None,
            interactions,
            children: Vec::new(),
        };
        self.node_stack.push(node);
    }

    /// Update the bounds of the current node (called during prepaint capture).
    pub fn set_current_bounds(&mut self, bounds: Bounds<Pixels>) {
        if let Some(node) = self.node_stack.last_mut() {
            node.bounds = Some(bounds);
        }
    }

    /// Update the content size of the current node.
    pub fn set_current_content_size(&mut self, size: Size<Pixels>) {
        if let Some(node) = self.node_stack.last_mut() {
            node.content_size = Some(size);
        }
    }

    /// Pop the current node off the stack, making it a child of the parent.
    /// If this is the last node on the stack, it becomes the root.
    pub fn pop_node(&mut self) {
        if self.node_stack.len() > 1 {
            let node = self.node_stack.pop().unwrap();
            if let Some(parent) = self.node_stack.last_mut() {
                parent.children.push(node);
            }
        }
    }

    /// Finish building and produce the display tree. Consumes the builder.
    pub fn finish(mut self) -> Option<DisplayTree> {
        // Drain remaining stack (in case of unbalanced push/pop).
        while self.node_stack.len() > 1 {
            self.pop_node();
        }
        self.node_stack.pop().map(|root| DisplayTree {
            frame_id: self.frame_id,
            viewport: self.viewport,
            root,
        })
    }

    fn add_leaf(&mut self, node: DisplayNode) {
        if let Some(parent) = self.node_stack.last_mut() {
            parent.children.push(node);
        }
    }
}

// ---------------------------------------------------------------------------
// Conversion helpers: GPUI types → DisplayTree types
// ---------------------------------------------------------------------------

impl DisplayColor {
    /// Convert from GPUI's Hsla color.
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

impl DisplayStyle {
    /// Convert from a resolved GPUI `Style`.
    pub fn from_gpui_style(style: &crate::Style) -> Self {
        Self {
            display: DisplayLayoutStyle::from_gpui_style(style),
            visual: DisplayVisualStyle::from_gpui_style(style),
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
            flex_grow: if style.flex_grow != 0.0 { Some(style.flex_grow) } else { None },
            flex_shrink: if style.flex_shrink != 1.0 { Some(style.flex_shrink) } else { None },
            width: DisplayLength::from_gpui_length(&style.size.width),
            height: DisplayLength::from_gpui_length(&style.size.height),
            min_width: DisplayLength::from_gpui_length(&style.min_size.width),
            max_width: DisplayLength::from_gpui_length(&style.max_size.width),
            min_height: DisplayLength::from_gpui_length(&style.min_size.height),
            max_height: DisplayLength::from_gpui_length(&style.max_size.height),
            padding: Some([
                DisplayLength::from_gpui_definite_length(&style.padding.top),
                DisplayLength::from_gpui_definite_length(&style.padding.right),
                DisplayLength::from_gpui_definite_length(&style.padding.bottom),
                DisplayLength::from_gpui_definite_length(&style.padding.left),
            ]),
            margin: None,
            gap: None,
            align_items: style.align_items.map(|v| format!("{:?}", v)),
            justify_content: style.justify_content.map(|v| format!("{:?}", v)),
            position: Some(format!("{:?}", style.position)),
            overflow_x: Some(format!("{:?}", style.overflow.x)),
            overflow_y: Some(format!("{:?}", style.overflow.y)),
        }
    }
}

impl DisplayLength {
    fn from_gpui_length(length: &crate::Length) -> Option<Self> {
        match length {
            crate::Length::Definite(d) => Self::from_gpui_definite_length_opt(d),
            crate::Length::Auto => Some(DisplayLength::Auto),
        }
    }

    fn from_gpui_definite_length(d: &crate::DefiniteLength) -> Self {
        Self::from_gpui_definite_length_opt(d).unwrap_or(DisplayLength::Px(0.0))
    }

    fn from_gpui_definite_length_opt(d: &crate::DefiniteLength) -> Option<Self> {
        match d {
            crate::DefiniteLength::Absolute(crate::AbsoluteLength::Pixels(px)) => {
                Some(DisplayLength::Px(px.0))
            }
            crate::DefiniteLength::Absolute(crate::AbsoluteLength::Rems(rems)) => {
                Some(DisplayLength::Px(rems.0 * 16.0))
            }
            crate::DefiniteLength::Fraction(f) => Some(DisplayLength::Percent(*f * 100.0)),
        }
    }
}

fn absolute_length_to_f32(len: &crate::AbsoluteLength) -> f32 {
    match len {
        crate::AbsoluteLength::Pixels(px) => px.0,
        crate::AbsoluteLength::Rems(rems) => rems.0 * 16.0,
    }
}

impl DisplayVisualStyle {
    fn from_gpui_style(style: &crate::Style) -> Self {
        Self {
            background: style.background.as_ref().and_then(|fill| match fill {
                crate::Fill::Color(bg) => Some(DisplayColor::from_hsla(bg.solid)),
            }),
            border_color: style.border_color.map(DisplayColor::from_hsla),
            border_widths: Some([
                absolute_length_to_f32(&style.border_widths.top),
                absolute_length_to_f32(&style.border_widths.right),
                absolute_length_to_f32(&style.border_widths.bottom),
                absolute_length_to_f32(&style.border_widths.left),
            ]),
            corner_radii: Some([
                absolute_length_to_f32(&style.corner_radii.top_left),
                absolute_length_to_f32(&style.corner_radii.top_right),
                absolute_length_to_f32(&style.corner_radii.bottom_right),
                absolute_length_to_f32(&style.corner_radii.bottom_left),
            ]),
            box_shadows: Vec::new(),
            opacity: style.opacity.filter(|&o| o < 1.0),
            cursor: style.mouse_cursor.map(|c| format!("{:?}", c)),
            visible: style.visibility != crate::Visibility::Hidden,
        }
    }
}

/// Compute a delta between two display trees.
///
/// Returns `None` if the trees are identical (no update needed).
/// The diffing algorithm walks both trees simultaneously, comparing nodes
/// by `DisplayNodeId`. When a subtree differs, it emits a `ReplaceNode`
/// patch. Targeted patches (style-only, text-only, scroll-only) are used
/// when only specific properties changed.
pub fn diff_display_trees(old: &DisplayTree, new: &DisplayTree) -> Option<DisplayTreeDelta> {
    let mut patches = Vec::new();
    diff_nodes(&old.root, &new.root, &mut patches);
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
    // Different node IDs or different element kinds mean full replacement.
    if old.id != new.id || std::mem::discriminant(&old.kind) != std::mem::discriminant(&new.kind) {
        patches.push(DisplayTreePatch::ReplaceNode {
            target: old.id,
            node: new.clone(),
        });
        return;
    }

    // Check for scroll offset changes on containers.
    if let (
        DisplayNodeKind::Container {
            scroll_offset: old_scroll,
            ..
        },
        DisplayNodeKind::Container {
            scroll_offset: new_scroll,
            ..
        },
    ) = (&old.kind, &new.kind)
    {
        if old_scroll != new_scroll {
            if let Some(offset) = new_scroll {
                patches.push(DisplayTreePatch::UpdateScrollOffset {
                    target: new.id,
                    offset: *offset,
                });
            }
        }
    }

    // Check for text content changes.
    if let (
        DisplayNodeKind::Text {
            content: old_content,
            runs: old_runs,
        },
        DisplayNodeKind::Text {
            content: new_content,
            runs: new_runs,
        },
    ) = (&old.kind, &new.kind)
    {
        if old_content != new_content || old_runs.len() != new_runs.len() {
            patches.push(DisplayTreePatch::UpdateText {
                target: new.id,
                content: new_content.clone(),
                runs: new_runs.clone(),
            });
        }
    }

    // Check for bounds changes.
    if old.bounds != new.bounds {
        if let Some(bounds) = &new.bounds {
            patches.push(DisplayTreePatch::UpdateBounds {
                target: new.id,
                bounds: *bounds,
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

    // Extra children in new tree: insertions.
    for i in common..new_len {
        patches.push(DisplayTreePatch::InsertChild {
            parent: new.id,
            index: i,
            node: new.children[i].clone(),
        });
    }

    // Extra children in old tree: removals (in reverse order to preserve indices).
    for i in (common..old_len).rev() {
        patches.push(DisplayTreePatch::RemoveChild {
            parent: old.id,
            index: i,
        });
    }
}

/// Action forwarding message: sent from browser to server when the user
/// interacts with an element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayAction {
    /// Which node was interacted with.
    pub node_id: DisplayNodeId,
    /// The serialized `GlobalElementId` path.
    pub element_id: Option<String>,
    /// What kind of interaction occurred.
    pub action: DisplayActionKind,
}

/// The specific interaction that occurred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayActionKind {
    /// Mouse click at a position relative to the element.
    Click {
        /// Position relative to element bounds.
        position: Point<Pixels>,
        /// Which mouse button.
        button: u8,
        /// Number of clicks (1 = single, 2 = double).
        click_count: u32,
        /// Modifier keys held.
        modifiers: DisplayModifiers,
    },

    /// Mouse down.
    MouseDown {
        /// Position relative to element bounds.
        position: Point<Pixels>,
        /// Which mouse button.
        button: u8,
        /// Modifier keys held.
        modifiers: DisplayModifiers,
    },

    /// Mouse up.
    MouseUp {
        /// Position relative to element bounds.
        position: Point<Pixels>,
        /// Which mouse button.
        button: u8,
        /// Modifier keys held.
        modifiers: DisplayModifiers,
    },

    /// Mouse hover state changed.
    Hover {
        /// Whether the mouse entered (true) or left (false) the element.
        entered: bool,
    },

    /// Scroll wheel event.
    Scroll {
        /// Scroll delta in pixels.
        delta: Point<Pixels>,
        /// Modifier keys held.
        modifiers: DisplayModifiers,
    },

    /// Key down event (forwarded when element has focus).
    KeyDown {
        /// Key name (e.g. "a", "Enter", "ArrowLeft").
        key: String,
        /// Modifier keys held.
        modifiers: DisplayModifiers,
    },

    /// Key up event.
    KeyUp {
        /// Key name.
        key: String,
        /// Modifier keys held.
        modifiers: DisplayModifiers,
    },

    /// Window/viewport resize.
    Resize {
        /// New viewport size.
        size: Size<Pixels>,
    },
}

/// Modifier key state for input events.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct DisplayModifiers {
    /// Ctrl key (or Cmd on macOS).
    pub control: bool,
    /// Alt/Option key.
    pub alt: bool,
    /// Shift key.
    pub shift: bool,
    /// Meta/Super/Windows key.
    pub meta: bool,
}
