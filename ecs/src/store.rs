use morphorm::{LayoutType, PositionType, Units};
use std::{collections::HashMap};

use crate::entity::Entity;

/// A storage struct representing a component store for an ECS
#[derive(Default)]
pub struct Store {
    pub visible: HashMap<Entity, bool>,

    pub layout_type: HashMap<Entity, LayoutType>,
    pub position_type: HashMap<Entity, PositionType>,

    pub left: HashMap<Entity, Units>,
    pub right: HashMap<Entity, Units>,
    pub top: HashMap<Entity, Units>,
    pub bottom: HashMap<Entity, Units>,

    pub min_left: HashMap<Entity, Units>,
    pub max_left: HashMap<Entity, Units>,
    pub min_right: HashMap<Entity, Units>,
    pub max_right: HashMap<Entity, Units>,
    pub min_top: HashMap<Entity, Units>,
    pub max_top: HashMap<Entity, Units>,
    pub min_bottom: HashMap<Entity, Units>,
    pub max_bottom: HashMap<Entity, Units>,

    pub width: HashMap<Entity, Units>,
    pub height: HashMap<Entity, Units>,
    pub min_width: HashMap<Entity, Units>,
    pub max_width: HashMap<Entity, Units>,
    pub min_height: HashMap<Entity, Units>,
    pub max_height: HashMap<Entity, Units>,

    pub child_left: HashMap<Entity, Units>,
    pub child_right: HashMap<Entity, Units>,
    pub child_top: HashMap<Entity, Units>,
    pub child_bottom: HashMap<Entity, Units>,
    pub row_between: HashMap<Entity, Units>,
    pub col_between: HashMap<Entity, Units>,

    pub grid_rows: HashMap<Entity, Vec<Units>>,
    pub grid_cols: HashMap<Entity, Vec<Units>>,

    pub row_index: HashMap<Entity, usize>,
    pub col_index: HashMap<Entity, usize>,
    pub row_span: HashMap<Entity, usize>,
    pub col_span: HashMap<Entity, usize>,

    pub border: HashMap<Entity, Units>,


    pub red: HashMap<Entity, u8>,
    pub green: HashMap<Entity, u8>,
    pub blue: HashMap<Entity, u8>,

    pub intrinsic_size: HashMap<Entity, Box<dyn Fn(f32) -> f32>>,
}
