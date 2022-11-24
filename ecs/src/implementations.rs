use std::collections::HashMap;
use std::iter::Rev;

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

    fn children<'t>(&self, tree: &'t Self::Tree) -> Self::ChildIter<'t> {
        let current_node = tree.get_first_child(self);
        ChildIterator {
            tree,
            current_node,
        }
    }

    fn layout_type(&self, store: &Self::Store) -> Option<LayoutType> {
        store.layout_type.get(self).copied()
    }

    fn position_type(&self, store: &Self::Store) -> Option<PositionType> {
        store.position_type.get(self).copied()
    }

    fn main(&self, store: &Self::Store) -> Option<Units> {
        store.main.get(self).copied()
    }

    fn cross(&self, store: &Self::Store) -> Option<Units> {
        store.cross.get(self).copied()
    }

    fn main_before(&self, store: &Self::Store) -> Option<Units> {
        store.main_before.get(self).copied()
    }

    fn main_after(&self, store: &Self::Store) -> Option<Units> {
        store.main_after.get(self).copied()
    }

    fn cross_before(&self, store: &Self::Store) -> Option<Units> {
        store.cross_before.get(self).copied()
    }

    fn cross_after(&self, store: &Self::Store) -> Option<Units> {
        store.cross_after.get(self).copied()
    }

    fn content_size(&self, store: &Self::Store, cross_size: f32) -> Option<f32> {
        if let Some(t) = store.content_size.get(self) {
            Some((t)(cross_size))
        } else {
            None
        }
    }

    fn child_main_before(&self, store: &Self::Store) -> Option<Units> {
        store.child_main_before.get(self).copied()
    }

    fn child_main_after(&self, store: &Self::Store) -> Option<Units> {
        store.child_main_after.get(self).copied()
    }

    fn child_cross_before(&self, store: &Self::Store) -> Option<Units> {
        store.child_cross_before.get(self).copied()
    }

    fn child_cross_after(&self, store: &Self::Store) -> Option<Units> {
        store.child_cross_after.get(self).copied()
    }
}

// impl<'a,'w> Node<'w> for &'a Entity
// where 'a: 'w
// {
//     type Data = Store;
// }

/*
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
*/

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub posx: f32,
    pub posy: f32,
    pub width: f32,
    pub height: f32,
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
}

impl NodeCache {
    pub fn add(&mut self, entity: Entity) {
        self.rect.insert(entity, Default::default());
    }

    pub fn bounds(&self, entity: Entity) -> Option<&Rect> {
        self.rect.get(&entity)
    }
}

impl Cache for NodeCache {
    type Node = Entity;

    fn width(&self, node: Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.width;
        }

        0.0
    }

    fn height(&self, node: Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.height;
        }

        0.0
    }

    fn posx(&self, node: Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.posx;
        }

        0.0
    }

    fn posy(&self, node: Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.posy;
        }

        0.0
    }

    fn set_width(&mut self, node: Self::Node, width: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.width = width;
        }
    }

    fn set_height(&mut self, node: Self::Node, height: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.height = height;
        }
    }

    fn set_posx(&mut self, node: Self::Node, posx: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.posx = posx;
        }
    }

    fn set_posy(&mut self, node: Self::Node, posy: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.posy = posy;
        }
    }
}
