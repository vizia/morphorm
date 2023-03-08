use std::collections::HashMap;

use morphorm::*;

use crate::entity::Entity;
use crate::store::Store;
use crate::tree::{ChildIterator, Tree};

impl Node for Entity {
    type Store = Store;
    type Tree = Tree;
    type ChildIter<'t> = ChildIterator<'t>;

    type CacheKey = Entity;

    fn key(&self) -> Self::CacheKey {
        *self
    }

    fn children<'t>(&self, tree: &'t Tree) -> Self::ChildIter<'t> {
        let current_node = tree.get_first_child(self);
        ChildIterator {
            tree,
            current_node,
        }
    }

    fn layout_type(&self, store: &Store) -> Option<LayoutType> {
        store.layout_type.get(self).copied()
    }

    fn position_type(&self, store: &Store) -> Option<PositionType> {
        store.position_type.get(self).copied()
    }

    fn width(&self, store: &Store) -> Option<Units> {
        store.width.get(self).copied()
    }

    fn height(&self, store: &Store) -> Option<Units> {
        store.height.get(self).copied()
    }

    fn left(&self, store: &Store) -> Option<Units> {
        store.left.get(self).copied()
    }

    fn right(&self, store: &Store) -> Option<Units> {
        store.right.get(self).copied()
    }

    fn top(&self, store: &Store) -> Option<Units> {
        store.top.get(self).copied()
    }

    fn bottom(&self, store: &Store) -> Option<Units> {
        store.bottom.get(self).copied()
    }

    fn content_size(&self, store: &Store, width: Option<f32>, height: Option<f32>) -> Option<(f32, f32)> {
        if let Some(t) = store.content_size.get(self) {
            Some((t)(store, width, height))
        } else {
            None
        }
    }

    fn child_left(&self, store: &Store) -> Option<Units> {
        store.child_left.get(self).copied()
    }

    fn child_right(&self, store: &Store) -> Option<Units> {
        store.child_right.get(self).copied()
    }

    fn child_top(&self, store: &Store) -> Option<Units> {
        store.child_top.get(self).copied()
    }

    fn child_bottom(&self, store: &Store) -> Option<Units> {
        store.child_bottom.get(self).copied()
    }

    fn row_between(&self, store: &Store) -> Option<Units> {
        store.row_between.get(self).copied()
    }

    fn col_between(&self, store: &Store) -> Option<Units> {
        store.col_between.get(self).copied()
    }

    fn min_width(&self, store: &Store) -> Option<Units> {
        store.min_width.get(self).copied()
    }

    fn max_width(&self, store: &Store) -> Option<Units> {
        store.max_width.get(self).copied()
    }

    fn min_height(&self, store: &Store) -> Option<Units> {
        store.min_height.get(self).copied()
    }

    fn max_height(&self, store: &Store) -> Option<Units> {
        store.max_height.get(self).copied()
    }
    
    fn min_left(&self, store: &Store) -> Option<Units> {
        store.min_left.get(self).copied()
    }

    fn max_left(&self, store: &Store) -> Option<Units> {
        store.max_left.get(self).copied()
    }

    fn min_right(&self, store: &Store) -> Option<Units> {
        store.min_right.get(self).copied()
    }

    fn max_right(&self, store: &Store) -> Option<Units> {
        store.max_right.get(self).copied()
    }

    fn min_top(&self, store: &Store) -> Option<Units> {
        store.min_top.get(self).copied()
    }

    fn max_top(&self, store: &Store) -> Option<Units> {
        store.max_top.get(self).copied()
    }

    fn min_bottom(&self, store: &Store) -> Option<Units> {
        store.min_bottom.get(self).copied()
    }

    fn max_bottom(&self, store: &Store) -> Option<Units> {
        store.max_bottom.get(self).copied()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub posx: f32,
    pub posy: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct NodeCache {
    // Computed Outputs
    pub rect: HashMap<Entity, Rect>,
}

impl NodeCache {
    pub fn add(&mut self, entity: Entity) {
        self.rect.insert(entity, Default::default());
    }

    pub fn bounds(&self, entity: Entity) -> Option<&Rect> {
        self.rect.get(&entity)
    }
}

impl Cache for NodeCache 
{
    type Node = Entity;

    fn width(&self, node: &Entity) -> f32 {
        if let Some(rect) = self.rect.get(node) {
            return rect.width;
        }

        0.0
    }

    fn height(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(node) {
            return rect.height;
        }

        0.0
    }

    fn posx(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(node) {
            return rect.posx;
        }

        0.0
    }

    fn posy(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(node) {
            return rect.posy;
        }

        0.0
    }

    fn set_width(&mut self, node: &Self::Node, width: f32) {
        if let Some(rect) = self.rect.get_mut(node) {
            rect.width = width;
        }
    }

    fn set_height(&mut self, node: &Self::Node, height: f32) {
        if let Some(rect) = self.rect.get_mut(node) {
            rect.height = height;
        }
    }

    fn set_posx(&mut self, node: &Self::Node, posx: f32) {
        if let Some(rect) = self.rect.get_mut(node) {
            rect.posx = posx;
        }
    }

    fn set_posy(&mut self, node: &Self::Node, posy: f32) {
        if let Some(rect) = self.rect.get_mut(node) {
            rect.posy = posy;
        }
    }
}
