use morphorm::{LayoutType, PositionType, Units};
use std::collections::HashMap;

use crate::{entity::Entity, TextWrap};

type ContentSizeType<S> = Box<dyn Fn(&S, Option<f32>, Option<f32>) -> (f32, f32)>;

/// A storage struct representing a component store for an ECS.
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
    pub col_between: HashMap<Entity, Units>,
    pub row_between: HashMap<Entity, Units>,

    pub content_size: HashMap<Entity, ContentSizeType<Self>>,

    pub text: HashMap<Entity, String>,
    pub text_wrap: HashMap<Entity, TextWrap>,

    pub text_context: femtovg::TextContext,
    pub font_id: Option<femtovg::FontId>,

    pub red: HashMap<Entity, u8>,
    pub green: HashMap<Entity, u8>,
    pub blue: HashMap<Entity, u8>,
}

impl Store {
    pub fn remove(&mut self, entity: &Entity) {
        self.visible.remove(entity);
        self.layout_type.remove(entity);
        self.position_type.remove(entity);
        self.left.remove(entity);
        self.right.remove(entity);
        self.top.remove(entity);
        self.bottom.remove(entity);
        self.min_left.remove(entity);
        self.max_left.remove(entity);
        self.min_right.remove(entity);
        self.max_right.remove(entity);
        self.min_top.remove(entity);
        self.max_top.remove(entity);
        self.min_bottom.remove(entity);
        self.max_bottom.remove(entity);
        self.width.remove(entity);
        self.height.remove(entity);
        self.min_width.remove(entity);
        self.max_width.remove(entity);
        self.min_height.remove(entity);
        self.max_height.remove(entity);
        self.child_left.remove(entity);
        self.child_right.remove(entity);
        self.child_top.remove(entity);
        self.child_bottom.remove(entity);
        self.col_between.remove(entity);
        self.row_between.remove(entity);
        self.content_size.remove(entity);
        self.text.remove(entity);
        self.text_wrap.remove(entity);
        self.red.remove(entity);
        self.green.remove(entity);
        self.blue.remove(entity);
    }
}
