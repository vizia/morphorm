// Part of a very simple ECS for demonstration purposes only.

use crate::{entity::Entity, TextWrap};
use morphorm::{LayoutType, PositionType, Units};
use slotmap::{SecondaryMap, SlotMap};

type ContentSizeType<S> = Box<dyn Fn(&S, Option<f32>, Option<f32>) -> (f32, f32)>;

/// A storage struct representing a component store for an ECS.
#[derive(Default)]
pub struct Store {
    pub visible: SlotMap<Entity, bool>,

    pub layout_type: SecondaryMap<Entity, LayoutType>,
    pub position_type: SecondaryMap<Entity, PositionType>,

    pub left: SecondaryMap<Entity, Units>,
    pub right: SecondaryMap<Entity, Units>,
    pub top: SecondaryMap<Entity, Units>,
    pub bottom: SecondaryMap<Entity, Units>,

    pub min_left: SecondaryMap<Entity, Units>,
    pub max_left: SecondaryMap<Entity, Units>,
    pub min_right: SecondaryMap<Entity, Units>,
    pub max_right: SecondaryMap<Entity, Units>,
    pub min_top: SecondaryMap<Entity, Units>,
    pub max_top: SecondaryMap<Entity, Units>,
    pub min_bottom: SecondaryMap<Entity, Units>,
    pub max_bottom: SecondaryMap<Entity, Units>,

    pub width: SecondaryMap<Entity, Units>,
    pub height: SecondaryMap<Entity, Units>,
    pub min_width: SecondaryMap<Entity, Units>,
    pub max_width: SecondaryMap<Entity, Units>,
    pub min_height: SecondaryMap<Entity, Units>,
    pub max_height: SecondaryMap<Entity, Units>,

    pub child_left: SecondaryMap<Entity, Units>,
    pub child_right: SecondaryMap<Entity, Units>,
    pub child_top: SecondaryMap<Entity, Units>,
    pub child_bottom: SecondaryMap<Entity, Units>,
    pub col_between: SecondaryMap<Entity, Units>,
    pub row_between: SecondaryMap<Entity, Units>,

    pub content_size: SecondaryMap<Entity, ContentSizeType<Self>>,

    pub text: SecondaryMap<Entity, String>,
    pub text_wrap: SecondaryMap<Entity, TextWrap>,

    pub text_context: femtovg::TextContext,
    pub font_id: Option<femtovg::FontId>,

    pub red: SecondaryMap<Entity, u8>,
    pub green: SecondaryMap<Entity, u8>,
    pub blue: SecondaryMap<Entity, u8>,
}

impl Store {
    pub fn remove(&mut self, entity: Entity) {
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
