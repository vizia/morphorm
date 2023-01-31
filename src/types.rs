/// The layout type determines how the nodes will position its parent-directed children.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    /// Stack child elements horizontally.
    Row,
    /// Stack child elements vertically.
    Column,
    /// Position child elements into specified rows and columns.
    Grid,
}

impl Default for LayoutType {
    fn default() -> Self {
        LayoutType::Row
    }
}

/// The position type determines whether a node will be positioned in-line with its siblings or seperately.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PositionType {
    /// Node is positioned relative to parent but ignores its siblings.
    SelfDirected,
    /// Node is positioned relative to parent and in-line with siblings.
    ParentDirected,
}

impl Default for PositionType {
    fn default() -> Self {
        PositionType::ParentDirected
    }
}

/// Units which describe spacing and size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Units {
    /// A number of logical pixels.
    Pixels(f32),
    /// A percentage of the parent dimension.
    Percentage(f32),
    /// A factor of the remaining free space.
    Stretch(f32),
    /// Automatically determine the value.
    Auto,
}

impl Default for Units {
    fn default() -> Self {
        Units::Auto
    }
}

impl Units {
    /// Converts the units to an f32 value
    pub fn to_px(&self, parent_value: f32, auto: f32) -> f32 {
        match self {
            &Units::Pixels(pixels) => pixels,
            &Units::Percentage(percentage) => (percentage / 100.0) * parent_value,
            &Units::Stretch(_) => auto,
            &Units::Auto => auto,
        }
    }

    /// Returns true if the value is in pixels
    pub fn is_pixels(&self) -> bool {
        match self {
            Units::Pixels(_) => true,
            _ => false,
        }
    }

    /// Returns true if the value is a percentage
    pub fn is_percentage(&self) -> bool {
        match self {
            Units::Percentage(_) => true,
            _ => false,
        }
    }

    /// Returns true if the value is a stretch factor
    pub fn is_stretch(&self) -> bool {
        match self {
            Units::Stretch(_) => true,
            _ => false,
        }
    }

    /// Returns true if the value is auto
    pub fn is_auto(&self) -> bool {
        match self {
            Units::Auto => true,
            _ => false,
        }
    }
}
