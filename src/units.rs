use bitflags::bitflags;

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

bitflags! {
    pub struct GeometryChanged: u8 {
        const CHANGE_POSX    = 0b00000001;
        const CHANGE_POSY    = 0b00000010;
        const CHANGE_WIDTH   = 0b00000100;
        const CHANGE_HEIGHT  = 0b00001000;
        const POSX_CHANGED   = 0b00010000;
        const POSY_CHANGED   = 0b00100000;
        const WIDTH_CHANGED  = 0b01000000;
        const HEIGHT_CHANGED = 0b10000000; 
    }     
}



// 

#[derive(Debug, Clone, Copy)]
pub struct Value {
    min: f32,
    val: f32,
    max: f32,
}

const MIN: f32 = -std::f32::MAX;
const MAX: f32 = std::f32::MAX;


#[derive(Debug, Clone, Copy)]
pub enum Units2 {
    Pixels(Value),
    Percentage(Value),
    Stretch(Value),
    Auto,
}

impl Units2 {
    pub fn pixels(val: f32) -> Self {
        Self::Pixels(Value {min: MIN, val, max: MAX})
    }

    pub fn percentage(val: f32) -> Self {
        Self::Pixels(Value {min: MIN, val, max: MAX})
    }

    pub fn stretch(val: f32) -> Self {
        Self::Pixels(Value {min: MIN, val, max: MAX})
    }

    pub fn auto() -> Self {
        Self::Auto
    }

    pub fn min(self, min: f32) -> Self {
        match self {
            Units2::Pixels(px) => {
                assert!(min < px.max, "min must be less than max");
                Units2::Pixels(Value {
                    min,
                    val: px.val,
                    max: px.max,
                })
            }

            Units2::Percentage(pc) => {
                assert!(min < pc.max, "min must be less than max");
                Units2::Percentage(Value {
                    min,
                    val: pc.val,
                    max: pc.max,
                })
            }

            Units2::Stretch(s) => {
                assert!(min < s.max, "min must be less than max");
                Units2::Stretch(Value {
                    min,
                    val: s.val,
                    max: s.max,
                })
            }

            Units2::Auto => {
                Units2::Auto
            }
        }
    }

    pub fn max(self, max: f32) -> Self {
        match self {
            Units2::Pixels(px) => {
                assert!(max > px.min, "max must be greater than min");
                Units2::Pixels(Value {
                    min: px.min,
                    val: px.val,
                    max,
                })
            }

            Units2::Percentage(pc) => {
                assert!(max > pc.min, "max must be greater than min");
                Units2::Percentage(Value {
                    min: pc.min,
                    val: pc.val,
                    max,
                })
            }

            Units2::Stretch(s) => {
                assert!(max > s.min, "max must be greater than min");
                Units2::Stretch(Value {
                    min: s.min,
                    val: s.val,
                    max,
                })
            }

            Units2::Auto => {
                Units2::Auto
            }
        }
    }

    pub fn clamp(&mut self) {
        match self {
            Units2::Pixels(px) => {
                px.val = px.val.clamp(px.min, px.max);
            }

            Units2::Percentage(pc) => {
                pc.val = pc.val.clamp(pc.min, pc.max);
            }

            Units2::Stretch(s) => {
                s.val = s.val.clamp(s.min, s.max);
            }

            _=> {}
        }
    }
}