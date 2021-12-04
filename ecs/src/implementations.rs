use std::collections::HashMap;

use morphorm::*;

use crate::entity::Entity;
use crate::store::Store;
use crate::tree::{ChildIterator, Tree};

impl<'a> Node<'a> for Entity {
    type Data = Store;

    fn layout_type(&self, store: &'_ Self::Data) -> Option<LayoutType> {
        store.layout_type.get(self).cloned()
    }

    /// Get the  position type of the node
    fn position_type(&self, store: &'_ Self::Data) -> Option<PositionType> {
        store.position_type.get(self).cloned()
    }

    fn width(&self, store: &'_ Self::Data) -> Option<Units> {
        store.width.get(self).cloned()
    }

    fn height(&self, store: &'_ Self::Data) -> Option<Units> {
        store.height.get(self).cloned()
    }

    fn left(&self, store: &'_ Self::Data) -> Option<Units> {
        store.left.get(self).cloned()
    }
    fn right(&self, store: &'_ Self::Data) -> Option<Units> {
        store.right.get(self).cloned()
    }
    fn top(&self, store: &'_ Self::Data) -> Option<Units> {
        store.top.get(self).cloned()
    }
    fn bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        store.bottom.get(self).cloned()
    }

    fn child_left(&self, store: &'_ Self::Data) -> Option<Units> {
        store.child_left.get(self).cloned()
    }

    fn child_right(&self, store: &'_ Self::Data) -> Option<Units> {
        store.child_right.get(self).cloned()
    }

    fn child_top(&self, store: &'_ Self::Data) -> Option<Units> {
        store.child_top.get(self).cloned()
    }

    fn child_bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        store.child_bottom.get(self).cloned()
    }

    fn min_left(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_left.get(self).cloned()
    }

    fn min_right(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_right.get(self).cloned()
    }

    fn min_top(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_top.get(self).cloned()
    }

    fn min_bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_bottom.get(self).cloned()
    }

    fn max_left(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_left.get(self).cloned()
    }

    fn max_right(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_right.get(self).cloned()
    }

    fn max_top(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_top.get(self).cloned()
    }

    fn max_bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_bottom.get(self).cloned()
    }

    fn min_width(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_width.get(self).cloned()
    }

    fn max_width(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_width.get(self).cloned()
    }

    fn min_height(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_height.get(self).cloned()
    }

    fn max_height(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_height.get(self).cloned()
    }

    fn row_between(&self, store: &'_ Self::Data) -> Option<Units> {
        store.row_between.get(self).cloned()
    }

    fn col_between(&self, store: &'_ Self::Data) -> Option<Units> {
        store.col_between.get(self).cloned()
    }

    fn grid_rows(&self, store: &'_ Self::Data) -> Option<Vec<Units>> {
        store.grid_rows.get(self).cloned()
    }

    fn grid_cols(&self, store: &'_ Self::Data) -> Option<Vec<Units>> {
        store.grid_cols.get(self).cloned()
    }

    fn row_index(&self, store: &'_ Self::Data) -> Option<usize> {
        store.row_index.get(self).cloned()
    }

    fn col_index(&self, store: &'_ Self::Data) -> Option<usize> {
        store.col_index.get(self).cloned()
    }

    fn row_span(&self, store: &'_ Self::Data) -> Option<usize> {
        store.row_span.get(self).cloned()
    }

    fn col_span(&self, store: &'_ Self::Data) -> Option<usize> {
        store.col_span.get(self).cloned()
    }

    fn border_left(&self, store: &'_ Self::Data) -> Option<Units> {
        store.border.get(self).cloned()
    }

    fn border_right(&self, store: &'_ Self::Data) -> Option<Units> {
        store.border.get(self).cloned()
    }

    fn border_top(&self, store: &'_ Self::Data) -> Option<Units> {
        store.border.get(self).cloned()
    }

    fn border_bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        store.border.get(self).cloned()
    }
}

impl<'a,'b> Node<'b> for &'a Entity 
where 'a: 'b
{
    type Data = Store;
}

impl<'a> Hierarchy<'a> for Tree {
    type Item = Entity;

    fn up_iter<F: FnMut(Self::Item)>(&'a self, mut f: F) {
        for entity in self.flatten().into_iter().rev() {
            (f)(entity);
        }
    }

    fn down_iter<F: FnMut(Self::Item)>(&'a self, mut f: F) {
        for entity in self.flatten().into_iter() {
            (f)(entity);
        }
    }

    fn child_iter<F: FnMut(Self::Item)>(&'a self, node: Self::Item, mut f: F) {
        let first_child = self.get_first_child(node);
        let iter = ChildIterator {
            tree: self,
            current_node: first_child,
        };

        for child in iter {
            (f)(child);
        }
    }

    fn parent(&self, node: Self::Item) -> Option<Self::Item> {
        if node.index() < self.parent.len() {
            return self.parent[node.index()]
        }

        None
    }

    fn is_first_child(&self, node: Self::Item) -> bool {
        if let Some(parent) = self.parent(node) {
            if let Some(first_child) = self.get_first_child(parent) {
                if first_child == node {
                    return true;
                } else {
                    return false;
                }
            }
        }

        false
    }

    fn is_last_child(&self, node: Self::Item) -> bool {
        if let Some(parent) = self.parent(node) {
            if let Some(mut temp) = self.get_first_child(parent) {
                while let Some(next_sibling) = self.get_next_sibling(temp) {
                    temp = next_sibling;
                }

                if temp == node {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect {
    pub posx: f32,
    pub posy: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Space {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct NodeCache {
    // Computed Outputs
    pub rect: HashMap<Entity, Rect>,

    // Intermediate Values
    space: HashMap<Entity, Space>,
    size: HashMap<Entity, Size>,

    child_width_max: HashMap<Entity, f32>,
    child_height_max: HashMap<Entity, f32>,
    child_width_sum: HashMap<Entity, f32>,
    child_height_sum: HashMap<Entity, f32>,

    grid_row_max: HashMap<Entity, f32>,
    grid_col_max: HashMap<Entity, f32>,

    horizontal_free_space: HashMap<Entity, f32>,
    horizontal_stretch_sum: HashMap<Entity, f32>,

    vertical_free_space: HashMap<Entity, f32>,
    vertical_stretch_sum: HashMap<Entity, f32>,

    stack_first_child: HashMap<Entity, bool>,
    stack_last_child: HashMap<Entity, bool>,

    geometry_changed: HashMap<Entity, GeometryChanged>,

    visible: HashMap<Entity, bool>,
}

impl NodeCache {
    pub fn add(&mut self, entity: Entity) {
        self.rect.insert(entity, Default::default());

        self.space.insert(entity, Default::default());

        self.child_width_max.insert(entity, Default::default());
        self.child_height_max.insert(entity, Default::default());
        self.child_width_sum.insert(entity, Default::default());
        self.child_height_sum.insert(entity, Default::default());

        self.grid_row_max.insert(entity, Default::default());
        self.grid_col_max.insert(entity, Default::default());

        self.horizontal_free_space
            .insert(entity, Default::default());
        self.horizontal_stretch_sum
            .insert(entity, Default::default());

        self.vertical_free_space.insert(entity, Default::default());
        self.vertical_stretch_sum.insert(entity, Default::default());

        self.stack_first_child.insert(entity, Default::default());
        self.stack_last_child.insert(entity, Default::default());

        self.size.insert(entity, Default::default());

        self.geometry_changed.insert(entity, Default::default());

        self.visible.insert(entity, true);
    }
}

impl Cache for NodeCache {
    type Item = Entity;

    fn visible(&self, node: Self::Item) -> bool {
        if let Some(value) = self.visible.get(&node) {
            return *value;
        }

        true
    }

    fn geometry_changed(&self, node: Self::Item) -> GeometryChanged {
        if let Some(geometry_changed) = self.geometry_changed.get(&node) {
            return *geometry_changed;
        }

        GeometryChanged::default()
    }

    fn set_geo_changed(&mut self, node: Self::Item, flag: GeometryChanged, value: bool) {
        if let Some(geometry_changed) = self.geometry_changed.get_mut(&node) {
            geometry_changed.set(flag, value);
        }
    }

    fn width(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.width;
        }

        0.0
    }

    fn height(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.height;
        }

        0.0
    }

    fn posx(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.posx;
        }

        0.0
    }

    fn posy(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.posy;
        }

        0.0
    }

    fn left(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.left;
        }

        0.0
    }

    fn right(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.right;
        }

        0.0
    }

    fn top(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.top;
        }

        0.0
    }

    fn bottom(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.bottom;
        }

        0.0
    }

    fn new_width(&self, node: Self::Item) -> f32 {
        if let Some(size) = self.size.get(&node) {
            return size.width;
        }

        0.0
    }

    fn new_height(&self, node: Self::Item) -> f32 {
        if let Some(size) = self.size.get(&node) {
            return size.height;
        }

        0.0
    }

    fn child_width_max(&self, node: Self::Item) -> f32 {
        *self.child_width_max.get(&node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_width_sum(&self, node: Self::Item) -> f32 {
        *self.child_width_sum.get(&node).unwrap()
    }

    /// Get the computed maximum width of the child nodes
    fn child_height_max(&self, node: Self::Item) -> f32 {
        *self.child_height_max.get(&node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_height_sum(&self, node: Self::Item) -> f32 {
        *self.child_height_sum.get(&node).unwrap()
    }

    /// Get the computed maximum grid row
    fn grid_row_max(&self, node: Self::Item) -> f32 {
        *self.grid_row_max.get(&node).unwrap()
    }

    /// Get the computed maximum grid column
    fn grid_col_max(&self, node: Self::Item) -> f32 {
        *self.grid_col_max.get(&node).unwrap()
    }

    // Setters
    fn set_visible(&mut self, node: Self::Item, value: bool) {
        *self.visible.get_mut(&node).unwrap() = value;
    }

    fn set_child_width_sum(&mut self, node: Self::Item, value: f32) {
        *self.child_width_sum.get_mut(&node).unwrap() = value;
    }

    fn set_child_height_sum(&mut self, node: Self::Item, value: f32) {
        *self.child_height_sum.get_mut(&node).unwrap() = value;
    }

    fn set_child_width_max(&mut self, node: Self::Item, value: f32) {
        *self.child_width_max.get_mut(&node).unwrap() = value;
    }

    fn set_child_height_max(&mut self, node: Self::Item, value: f32) {
        *self.child_height_max.get_mut(&node).unwrap() = value;
    }

    fn horizontal_free_space(&self, node: Self::Item) -> f32 {
        *self.horizontal_free_space.get(&node).unwrap()
    }
    fn set_horizontal_free_space(&mut self, node: Self::Item, value: f32) {
        *self.horizontal_free_space.get_mut(&node).unwrap() = value;
    }
    fn vertical_free_space(&self, node: Self::Item) -> f32 {
        *self.vertical_free_space.get(&node).unwrap()
    }
    fn set_vertical_free_space(&mut self, node: Self::Item, value: f32) {
        *self.vertical_free_space.get_mut(&node).unwrap() = value;
    }

    fn horizontal_stretch_sum(&self, node: Self::Item) -> f32 {
        *self.horizontal_stretch_sum.get(&node).unwrap()
    }
    fn set_horizontal_stretch_sum(&mut self, node: Self::Item, value: f32) {
        *self.horizontal_stretch_sum.get_mut(&node).unwrap() = value;
    }
    fn vertical_stretch_sum(&self, node: Self::Item) -> f32 {
        *self.vertical_stretch_sum.get(&node).unwrap()
    }
    fn set_vertical_stretch_sum(&mut self, node: Self::Item, value: f32) {
        *self.vertical_stretch_sum.get_mut(&node).unwrap() = value;
    }

    fn set_grid_row_max(&mut self, node: Self::Item, value: f32) {
        *self.grid_row_max.get_mut(&node).unwrap() = value;
    }

    fn set_grid_col_max(&mut self, node: Self::Item, value: f32) {
        *self.grid_row_max.get_mut(&node).unwrap() = value;
    }

    fn set_width(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.width = value;
        }
    }
    fn set_height(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.height = value;
        }
    }
    fn set_posx(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.posx = value;
        }
    }
    fn set_posy(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.posy = value;
        }
    }

    fn set_left(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.left = value;
        }
    }

    fn set_right(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.right = value;
        }
    }

    fn set_top(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.top = value;
        }
    }

    fn set_bottom(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.bottom = value;
        }
    }

    fn set_new_width(&mut self, node: Self::Item, value: f32) {
        if let Some(size) = self.size.get_mut(&node) {
            size.width = value;
        }
    }

    fn set_new_height(&mut self, node: Self::Item, value: f32) {
        if let Some(size) = self.size.get_mut(&node) {
            size.height = value;
        }
    }

    fn stack_first_child(&self, node: Self::Item) -> bool {
        *self.stack_first_child.get(&node).unwrap()
    }

    fn set_stack_first_child(&mut self, node: Self::Item, value: bool) {
        *self.stack_first_child.get_mut(&node).unwrap() = value;
    }

    fn stack_last_child(&self, node: Self::Item) -> bool {
        *self.stack_last_child.get(&node).unwrap()
    }

    fn set_stack_last_child(&mut self, node: Self::Item, value: bool) {
        *self.stack_last_child.get_mut(&node).unwrap() = value;
    }
}
