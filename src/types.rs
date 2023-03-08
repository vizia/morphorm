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
    // Helper function for selecting between values depending on the layout type.
    pub(crate) fn select<T: Default>(&self, first: Option<T>, second: Option<T>) -> T {
        match self {
            LayoutType::Row => first.unwrap_or_default(),
            LayoutType::Column => second.unwrap_or_default(),
        }
    }
}

/// The position type determines whether a node will be positioned in-line with its siblings or seperately.
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
    Percentage(f32),
    /// A factor of the remaining free space.
    Stretch(f32),
    /// Automatically determine the value.
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

    /// Returns true if the value is in pixels
    pub fn is_pixels(&self) -> bool {
        matches!(self, Units::Pixels(_))
    }

    /// Returns true if the value is a percentage
    pub fn is_percentage(&self) -> bool {
        matches!(self, Units::Percentage(_))
    }

    /// Returns true if the value is a stretch factor
    pub fn is_stretch(&self) -> bool {
        matches!(self, Units::Stretch(_))
    }

    /// Returns true if the value is auto
    pub fn is_auto(&self) -> bool {
        self == &Units::Auto
    }
}
