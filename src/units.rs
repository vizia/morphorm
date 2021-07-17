/// The layout type determines how nodes will be positioned when directed by the parent
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    /// Stack child elements horizontally
    Row,
    /// Stack child elements vertically
    Column,
    /// Position child elements into specified rows and columns
    Grid,
}

impl Default for LayoutType {
    fn default() -> Self {
        LayoutType::Column
    }
}

/// The position type determines whether a node will be positioned in-line with its siblings or seperate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PositionType {
    /// Node is positioned relative to parent but ignores its siblings
    SelfDirected,
    /// Node is positioned relative to parent and in-line with siblings
    ParentDirected,
}

impl Default for PositionType {
    fn default() -> Self {
        PositionType::ParentDirected
    }
}

/// Units which describe spacing and size
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Units {
    Pixels(f32),
    Percentage(f32),
    Stretch(f32),
    Auto,
}

impl Default for Units {
    fn default() -> Self {
        Units::Auto
    }
}

impl Units {
    pub fn value_or(&self, parent_value: f32, auto: f32) -> f32 {
        match self {
            &Units::Pixels(pixels) => pixels,
            &Units::Percentage(percentage) => (percentage / 100.0) * parent_value,
            &Units::Stretch(_) => auto,
            &Units::Auto => auto,
        }
    }
}
