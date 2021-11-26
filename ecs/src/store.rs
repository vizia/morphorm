use morphorm::{LayoutType, PositionType, Units};
use std::{collections::HashMap};

use crate::entity::Entity;

/// A storage struct representing a component store for an ECS
#[derive(Default)]
pub struct Store {
    pub visible: HashMap<Entity, bool>,

    pub layout_type: HashMap<Entity, LayoutType>,
    pub position_type: HashMap<Entity, PositionType>,

    pub main_before: HashMap<Entity, Units>,
    pub main_after: HashMap<Entity, Units>,
    pub cross_before: HashMap<Entity, Units>,
    pub cross_after: HashMap<Entity, Units>,

    pub min_main_before: HashMap<Entity, Units>,
    pub max_main_before: HashMap<Entity, Units>,
    pub min_main_after: HashMap<Entity, Units>,
    pub max_main_after: HashMap<Entity, Units>,
    pub min_cross_before: HashMap<Entity, Units>,
    pub max_cross_before: HashMap<Entity, Units>,
    pub min_cross_after: HashMap<Entity, Units>,
    pub max_cross_after: HashMap<Entity, Units>,

    pub main: HashMap<Entity, Units>,
    pub cross: HashMap<Entity, Units>,
    pub min_main: HashMap<Entity, Units>,
    pub max_main: HashMap<Entity, Units>,
    pub min_cross: HashMap<Entity, Units>,
    pub max_cross: HashMap<Entity, Units>,

    pub child_main_before: HashMap<Entity, Units>,
    pub child_main_after: HashMap<Entity, Units>,
    pub child_cross_before: HashMap<Entity, Units>,
    pub child_cross_after: HashMap<Entity, Units>,
    pub main_between: HashMap<Entity, Units>,
    pub cross_between: HashMap<Entity, Units>,

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

    pub intrinsic_size: HashMap<Entity, Box<dyn Fn(&Store, f32) -> f32>>,

    pub text: HashMap<Entity, String>,

    pub text_context: femtovg::TextContext,
    pub font_id: Option<femtovg::FontId>,
}
