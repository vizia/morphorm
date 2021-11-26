use std::collections::HashMap;
use std::iter::Rev;
use std::slice::SliceIndex;

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

    fn main(&self, store: &'_ Self::Data) -> Option<Units> {
        store.main.get(self).cloned()
    }

    fn cross(&self, store: &'_ Self::Data) -> Option<Units> {
        store.cross.get(self).cloned()
    }

    fn main_before(&self, store: &'_ Self::Data) -> Option<Units> {
        store.main_before.get(self).cloned()
    }

    fn main_after(&self, store: &'_ Self::Data) -> Option<Units> {
        store.main_after.get(self).cloned()
    }

    fn cross_before(&self, store: &'_ Self::Data) -> Option<Units> {
        store.cross_before.get(self).cloned()
    }

    fn cross_after(&self, store: &'_ Self::Data) -> Option<Units> {
        store.cross_after.get(self).cloned()
    }

    fn child_main_before(&self, store: &'_ Self::Data) -> Option<Units> {
        store.child_main_before.get(self).cloned()
    }

    fn child_main_after(&self, store: &'_ Self::Data) -> Option<Units> {
        store.child_main_after.get(self).cloned()
    }

    fn child_cross_before(&self, store: &'_ Self::Data) -> Option<Units> {
        store.child_cross_before.get(self).cloned()
    }

    fn child_cross_after(&self, store: &'_ Self::Data) -> Option<Units> {
        store.child_cross_after.get(self).cloned()
    }

    fn min_main_before(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_main_before.get(self).cloned()
    }

    fn min_main_after(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_main_after.get(self).cloned()
    }

    fn min_cross_before(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_cross_before.get(self).cloned()
    }

    fn min_cross_after(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_cross_after.get(self).cloned()
    }

    fn max_main_before(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_main_before.get(self).cloned()
    }

    fn max_main_after(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_main_after.get(self).cloned()
    }

    fn max_cross_before(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_cross_before.get(self).cloned()
    }

    fn max_cross_after(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_cross_after.get(self).cloned()
    }

    fn min_main(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_main.get(self).cloned()
    }

    fn max_main(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_main.get(self).cloned()
    }

    fn min_cross(&self, store: &'_ Self::Data) -> Option<Units> {
        store.min_cross.get(self).cloned()
    }

    fn max_cross(&self, store: &'_ Self::Data) -> Option<Units> {
        store.max_cross.get(self).cloned()
    }

    fn main_between(&self, store: &'_ Self::Data) -> Option<Units> {
        store.main_between.get(self).cloned()
    }

    fn cross_between(&self, store: &'_ Self::Data) -> Option<Units> {
        store.cross_between.get(self).cloned()
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

    fn intrinsic_size(&self, store: &'_ Self::Data, size: f32) -> Option<f32> {
        store.intrinsic_size.get(self).map(|intrinsic_size| (intrinsic_size)(&store, size))
    }
}

impl<'a,'b> Node<'b> for &'a Entity 
where 'a: 'b
{
    type Data = Store;
}

impl<'a> Hierarchy<'a> for Tree {
    type Item = Entity;
    type DownIter = std::vec::IntoIter<Entity>;
    type UpIter = Rev<std::vec::IntoIter<Entity>>;
    type ChildIter = ChildIterator<'a>;

    fn up_iter(&'a self) -> Self::UpIter {
        self.flatten().into_iter().rev()
    }

    fn down_iter(&'a self) -> Self::DownIter {
        self.flatten().into_iter()
    }

    fn child_iter(&'a self, node: Self::Item) -> Self::ChildIter {
        let first_child = self.get_first_child(node);
        ChildIterator {
            tree: self,
            current_node: first_child,
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

    child_main_max: HashMap<Entity, f32>,
    child_cross_max: HashMap<Entity, f32>,
    child_main_sum: HashMap<Entity, f32>,
    child_cross_sum: HashMap<Entity, f32>,

    grid_row_max: HashMap<Entity, f32>,
    grid_col_max: HashMap<Entity, f32>,

    main_free_space: HashMap<Entity, f32>,
    main_stretch_sum: HashMap<Entity, f32>,

    cross_free_space: HashMap<Entity, f32>,
    cross_stretch_sum: HashMap<Entity, f32>,

    stack_first_child: HashMap<Entity, bool>,
    stack_last_child: HashMap<Entity, bool>,

    geometry_changed: HashMap<Entity, GeometryChanged>,

    visible: HashMap<Entity, bool>,
}

impl NodeCache {
    pub fn add(&mut self, entity: Entity) {
        self.rect.insert(entity, Default::default());

        self.space.insert(entity, Default::default());

        self.child_main_max.insert(entity, Default::default());
        self.child_cross_max.insert(entity, Default::default());
        self.child_main_sum.insert(entity, Default::default());
        self.child_cross_sum.insert(entity, Default::default());

        self.grid_row_max.insert(entity, Default::default());
        self.grid_col_max.insert(entity, Default::default());

        self.main_free_space
            .insert(entity, Default::default());
        self.main_stretch_sum
            .insert(entity, Default::default());

        self.cross_free_space.insert(entity, Default::default());
        self.cross_stretch_sum.insert(entity, Default::default());

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

    fn main_before(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.left;
        }

        0.0
    }

    fn main_after(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.right;
        }

        0.0
    }

    fn cross_before(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.top;
        }

        0.0
    }

    fn cross_after(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.bottom;
        }

        0.0
    }

    fn new_main(&self, node: Self::Item) -> f32 {
        if let Some(size) = self.size.get(&node) {
            return size.width;
        }

        0.0
    }

    fn new_cross(&self, node: Self::Item) -> f32 {
        if let Some(size) = self.size.get(&node) {
            return size.height;
        }

        0.0
    }

    fn child_main_max(&self, node: Self::Item) -> f32 {
        *self.child_main_max.get(&node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_main_sum(&self, node: Self::Item) -> f32 {
        *self.child_main_sum.get(&node).unwrap()
    }

    /// Get the computed maximum width of the child nodes
    fn child_cross_max(&self, node: Self::Item) -> f32 {
        *self.child_cross_max.get(&node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_cross_sum(&self, node: Self::Item) -> f32 {
        *self.child_cross_sum.get(&node).unwrap()
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

    fn set_child_main_sum(&mut self, node: Self::Item, value: f32) {
        *self.child_main_sum.get_mut(&node).unwrap() = value;
    }

    fn set_child_cross_sum(&mut self, node: Self::Item, value: f32) {
        *self.child_cross_sum.get_mut(&node).unwrap() = value;
    }

    fn set_child_main_max(&mut self, node: Self::Item, value: f32) {
        *self.child_main_max.get_mut(&node).unwrap() = value;
    }

    fn set_child_cross_max(&mut self, node: Self::Item, value: f32) {
        *self.child_cross_max.get_mut(&node).unwrap() = value;
    }

    fn main_free_space(&self, node: Self::Item) -> f32 {
        *self.main_free_space.get(&node).unwrap()
    }
    fn set_main_free_space(&mut self, node: Self::Item, value: f32) {
        *self.main_free_space.get_mut(&node).unwrap() = value;
    }
    fn cross_free_space(&self, node: Self::Item) -> f32 {
        *self.cross_free_space.get(&node).unwrap()
    }
    fn set_cross_free_space(&mut self, node: Self::Item, value: f32) {
        *self.cross_free_space.get_mut(&node).unwrap() = value;
    }

    fn main_stretch_sum(&self, node: Self::Item) -> f32 {
        *self.main_stretch_sum.get(&node).unwrap()
    }
    fn set_main_stretch_sum(&mut self, node: Self::Item, value: f32) {
        *self.main_stretch_sum.get_mut(&node).unwrap() = value;
    }
    fn cross_stretch_sum(&self, node: Self::Item) -> f32 {
        *self.cross_stretch_sum.get(&node).unwrap()
    }
    fn set_cross_stretch_sum(&mut self, node: Self::Item, value: f32) {
        *self.cross_stretch_sum.get_mut(&node).unwrap() = value;
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

    fn set_main_before(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.left = value;
        }
    }

    fn set_main_after(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.right = value;
        }
    }

    fn set_cross_before(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.top = value;
        }
    }

    fn set_cross_after(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.bottom = value;
        }
    }

    fn set_new_main(&mut self, node: Self::Item, value: f32) {
        if let Some(size) = self.size.get_mut(&node) {
            size.width = value;
        }
    }

    fn set_new_cross(&mut self, node: Self::Item, value: f32) {
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
