use crate::{LayoutType, Node};

/// The `Cache` is a store which contains the computed size and position of nodes
/// after a layout calculation.
///
/// The `Node` associated type, which implements the [`Node`](crate::Node) trait, provides
/// a [`CacheKey'](crate::Node::CacheKey) associated type which can be used as key for storage types
/// within the cache if the `Node` type itself cannot be used. For example, as the key to a hashmap.
pub trait Cache {
    /// A type which represents a layout node and implments the [`Node`](crate::Node) trait.
    type Node: Node;
    /// Returns the cached width of the given node.
    fn width(&self, node: &Self::Node) -> f32;
    /// Returns the cached height of the given node.
    fn height(&self, node: &Self::Node) -> f32;
    /// Returns the cached horizontal position of the given node.
    fn posx(&self, node: &Self::Node) -> f32;
    /// Returns the cached vertical position of the given node.
    fn posy(&self, node: &Self::Node) -> f32;

    /// Sets the cached width of the given node.
    fn set_width(&mut self, node: &Self::Node, width: f32);
    /// Sets the cached height of the given node.
    fn set_height(&mut self, node: &Self::Node, height: f32);
    /// Sets the cached horizontal position of the given node.
    fn set_posx(&mut self, node: &Self::Node, posx: f32);
    /// Sets the cached vertical position of the given node.
    fn set_posy(&mut self, node: &Self::Node, posy: f32);
}

/// Helper trait for getting/setting node size/position in a direction agnostic way.
pub(crate) trait CacheExt: Cache {
    /// Set the computed main size of the `node` in the cache. Width for a row parent layout and height for a column parent playout.
    fn set_main(&mut self, node: &Self::Node, parent_layout_type: LayoutType, main: f32) {
        match parent_layout_type {
            LayoutType::Row => self.set_width(node, main),
            LayoutType::Column => self.set_height(node, main),
        }
    }

    /// Set the computed cross size of the `node` in the cache. Height for a row parent layout and width for a column parent playout.
    fn set_cross(&mut self, node: &Self::Node, parent_layout_type: LayoutType, cross: f32) {
        match parent_layout_type {
            LayoutType::Row => self.set_height(node, cross),
            LayoutType::Column => self.set_width(node, cross),
        }
    }

    /// Set the computed size of the `node` in the cache.
    fn set_size(&mut self, node: &Self::Node, parent_layout_type: LayoutType, main: f32, cross: f32) {
        self.set_main(node, parent_layout_type, main);
        self.set_cross(node, parent_layout_type, cross);
    }

    /// Set the computed main position of the `node` in the cache. Posx for a row parent layout and posy for a column parent playout.
    fn set_main_pos(&mut self, node: &Self::Node, parent_layout_type: LayoutType, main_pos: f32) {
        match parent_layout_type {
            LayoutType::Row => self.set_posx(node, main_pos),
            LayoutType::Column => self.set_posy(node, main_pos),
        }
    }

    /// Set the computed cross position of the `node` in the cache. Posy for a row parent layout and posx for a column parent playout.
    fn set_cross_pos(&mut self, node: &Self::Node, parent_layout_type: LayoutType, cross_pos: f32) {
        match parent_layout_type {
            LayoutType::Row => self.set_posy(node, cross_pos),
            LayoutType::Column => self.set_posx(node, cross_pos),
        }
    }

    /// Set the computed position of the `node` in the cache.
    fn set_pos(&mut self, node: &Self::Node, parent_layout_type: LayoutType, main_pos: f32, cross_pos: f32) {
        self.set_main_pos(node, parent_layout_type, main_pos);
        self.set_cross_pos(node, parent_layout_type, cross_pos);
    }
}

impl<C: Cache> CacheExt for C {}
