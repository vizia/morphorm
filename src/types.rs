/// The layout type determines how the nodes will position its parent-directed children.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType {
    /// Stack child elements horizontally.
    Row,
    /// Stack child elements vertically.
    #[default]
    Column,
}

impl LayoutType {
    // Helper function for selecting between optional values depending on the layout type.
    pub(crate) fn select_unwrap<T: Default, S>(
        &self,
        s: S,
        first: impl FnOnce(S) -> Option<T>,
        second: impl FnOnce(S) -> Option<T>,
    ) -> T {
        match self {
            LayoutType::Row => first(s).unwrap_or_default(),
            LayoutType::Column => second(s).unwrap_or_default(),
        }
    }
}

/// The position type determines whether a node will be positioned in-line with its siblings or out-of-line / independently of its siblings.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionType {
    /// Node is positioned relative to parent but ignores its siblings.
    SelfDirected,
    /// Node is positioned relative to parent and in-line with siblings.
    #[default]
    ParentDirected,
}

/// Units which describe spacing and size.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Units {
    /// A number of logical pixels.
    Pixels(f32),
    /// A percentage of the parent dimension.
    ///
    /// A percentage of the parent's width when applied to left, width, right properties.
    /// A percentage of the parent's height when applied to top, height, bottom properties.
    Percentage(f32),
    /// A factor of the remaining free space.
    ///
    /// The remaining free space is the parent space minus the space and size of any fixed-size nodes in that axis.
    /// The remaining free space is then shared between any stretch nodes based on the ratio of their stretch factors.
    ///
    /// For example, given two stretch nodes with factors of 1.0 and 2.0 respectively. The first will occupy 1/3 of the
    /// remaining free space while the second will occupy 2/3 of the remaining free space.
    Stretch(f32),
    /// Automatically determine the value.
    ///
    /// When applied to space (left, right, top, bottom) the spacing may be overridden by the parent's child-space on the same side.
    /// For example, a node in a column with `Auto` left space, with a parent which has Pixel(100.0) child-left space, will get a left spacing of 100px.
    ///
    /// When applied to size (width, height) Auto will either size to fit its children, or if there are no children
    /// the node will be sized based on the [`content_size`](crate::Node::content_size) property of the node.
    #[default]
    Auto,
}

impl Units {
    /// Returns the units converted to pixels or a provided default.
    pub fn to_px(&self, parent_value: f32, default: f32) -> f32 {
        match self {
            Units::Pixels(pixels) => *pixels,
            Units::Percentage(percentage) => (percentage / 100.0) * parent_value,
            Units::Stretch(_) => default,
            Units::Auto => default,
        }
    }

    pub fn to_px_clamped(&self, parent_value: f32, default: f32, min: Units, max: Units) -> f32 {
        let min = min.to_px(parent_value, f32::MIN);
        let max = max.to_px(parent_value, f32::MAX);
        match self {
            Units::Pixels(pixels) => pixels.clamp(min, max),
            Units::Percentage(percentage) => ((percentage / 100.0) * parent_value).clamp(min, max),
            Units::Stretch(_) => default.clamp(min, max),
            Units::Auto => default.clamp(min, max),
        }
    }

    pub fn clamp(&self, min: Units, max: Units) -> Self {
        match (self, min, max) {
            (Units::Pixels(val), Units::Pixels(min), Units::Pixels(max)) => Units::Pixels(val.min(max).max(min)),
            (Units::Percentage(val), Units::Percentage(min), Units::Percentage(max)) => {
                Units::Percentage(val.min(max).max(min))
            }
            (Units::Stretch(val), Units::Stretch(min), Units::Stretch(max)) => Units::Stretch(val.min(max).max(min)),
            _ => *self,
        }
    }

    /// Returns true if the value is in pixels.
    pub fn is_pixels(&self) -> bool {
        matches!(self, Units::Pixels(_))
    }

    /// Returns true if the value is a percentage.
    pub fn is_percentage(&self) -> bool {
        matches!(self, Units::Percentage(_))
    }

    /// Returns true if the value is a stretch factor.
    pub fn is_stretch(&self) -> bool {
        matches!(self, Units::Stretch(_))
    }

    /// Returns true if the value is auto.
    pub fn is_auto(&self) -> bool {
        self == &Units::Auto
    }
}

/// A type which represents the computed size of a node after [`layout`](crate::Node::layout).
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Size {
    /// The computed size on the main axis.
    pub main: f32,
    /// The computed size on the cross axis.
    pub cross: f32,
}
