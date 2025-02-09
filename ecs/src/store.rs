// Part of a very simple ECS for demonstration purposes only.

use crate::{entity::Entity, TextWrap};
use morphorm::{LayoutType, PositionType, Units, Alignment};
use slotmap::SecondaryMap;

type ContentSizeType<S> = Box<dyn Fn(&S, Option<f32>, Option<f32>) -> (f32, f32)>;

/// A storage struct representing a component store for an ECS.
#[derive(Default)]
pub struct Store {
    pub visible: SecondaryMap<Entity, bool>,

    pub layout_type: SecondaryMap<Entity, LayoutType>,
    pub position_type: SecondaryMap<Entity, PositionType>,
    pub alignment: SecondaryMap<Entity, Alignment>,
    pub absolute_auto: SecondaryMap<Entity, bool>,

    pub grid_columns: SecondaryMap<Entity, Vec<Units>>,
    pub grid_rows: SecondaryMap<Entity, Vec<Units>>,
    pub column_start: SecondaryMap<Entity, usize>,
    pub row_start: SecondaryMap<Entity, usize>,
    pub column_span: SecondaryMap<Entity, usize>,
    pub row_span: SecondaryMap<Entity, usize>,

    pub vertical_scroll: SecondaryMap<Entity, f32>,
    pub horizontal_scroll: SecondaryMap<Entity, f32>,

    pub left: SecondaryMap<Entity, Units>,
    pub right: SecondaryMap<Entity, Units>,
    pub top: SecondaryMap<Entity, Units>,
    pub bottom: SecondaryMap<Entity, Units>,

    pub width: SecondaryMap<Entity, Units>,
    pub height: SecondaryMap<Entity, Units>,
    pub min_width: SecondaryMap<Entity, Units>,
    pub max_width: SecondaryMap<Entity, Units>,
    pub min_height: SecondaryMap<Entity, Units>,
    pub max_height: SecondaryMap<Entity, Units>,

    pub min_horizontal_gap: SecondaryMap<Entity, Units>,
    pub min_vertical_gap: SecondaryMap<Entity, Units>,
    pub max_horizontal_gap: SecondaryMap<Entity, Units>,
    pub max_vertical_gap: SecondaryMap<Entity, Units>,

    pub padding_left: SecondaryMap<Entity, Units>,
    pub padding_right: SecondaryMap<Entity, Units>,
    pub padding_top: SecondaryMap<Entity, Units>,
    pub padding_bottom: SecondaryMap<Entity, Units>,
    pub horizontal_gap: SecondaryMap<Entity, Units>,
    pub vertical_gap: SecondaryMap<Entity, Units>,

    pub content_size: SecondaryMap<Entity, ContentSizeType<Self>>,

    pub text: SecondaryMap<Entity, String>,
    pub text_wrap: SecondaryMap<Entity, TextWrap>,

    pub text_context: femtovg::TextContext,
    pub font_id: Option<femtovg::FontId>,

    pub red: SecondaryMap<Entity, u8>,
    pub green: SecondaryMap<Entity, u8>,
    pub blue: SecondaryMap<Entity, u8>,

    pub border_left: SecondaryMap<Entity, Units>,
    pub border_right: SecondaryMap<Entity, Units>,
    pub border_top: SecondaryMap<Entity, Units>,
    pub border_bottom: SecondaryMap<Entity, Units>,
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
        self.vertical_scroll.remove(entity);
        self.horizontal_scroll.remove(entity);
        self.width.remove(entity);
        self.height.remove(entity);
        self.min_width.remove(entity);
        self.max_width.remove(entity);
        self.min_height.remove(entity);
        self.max_height.remove(entity);
        self.min_horizontal_gap.remove(entity);
        self.max_horizontal_gap.remove(entity);
        self.min_vertical_gap.remove(entity);
        self.max_vertical_gap.remove(entity);
        self.padding_left.remove(entity);
        self.padding_right.remove(entity);
        self.padding_top.remove(entity);
        self.padding_bottom.remove(entity);
        self.horizontal_gap.remove(entity);
        self.vertical_gap.remove(entity);
        self.content_size.remove(entity);
        self.text.remove(entity);
        self.text_wrap.remove(entity);
        self.red.remove(entity);
        self.green.remove(entity);
        self.blue.remove(entity);
        self.border_left.remove(entity);
        self.border_right.remove(entity);
        self.border_top.remove(entity);
        self.border_bottom.remove(entity);
    }

    pub fn clear(&mut self) {
        self.visible.clear();
        self.layout_type.clear();
        self.position_type.clear();
        self.left.clear();
        self.right.clear();
        self.top.clear();
        self.bottom.clear();
        self.vertical_scroll.clear();
        self.horizontal_scroll.clear();
        self.width.clear();
        self.height.clear();
        self.min_width.clear();
        self.max_width.clear();
        self.min_height.clear();
        self.max_height.clear();
        self.min_horizontal_gap.clear();
        self.max_horizontal_gap.clear();
        self.min_vertical_gap.clear();
        self.max_vertical_gap.clear();
        self.padding_left.clear();
        self.padding_right.clear();
        self.padding_top.clear();
        self.padding_bottom.clear();
        self.horizontal_gap.clear();
        self.vertical_gap.clear();
        self.content_size.clear();
        self.text.clear();
        self.text_wrap.clear();
        self.red.clear();
        self.green.clear();
        self.blue.clear();
        self.border_left.clear();
        self.border_right.clear();
        self.border_top.clear();
        self.border_bottom.clear();
    }
}
