use crate::{LayoutType, Node};

/// The `Cache` is a store which contains the computed size and position of nodes
/// after a layout calculation.
///
/// The `Node` associated type, which implements the [`Node`](crate::Node) trait, provides
/// a [`CacheKey`](crate::Node::CacheKey) associated type which can be used as key for storage types
/// within the cache if the `Node` type itself cannot be used. For example, as the key to a hashmap/slotmap.
pub trait Cache {
    /// A type which represents a layout node and implements the [`Node`](crate::Node) trait.
    type Node: Node;
    /// Returns the cached width of the given node.
    fn width(&self, node: &Self::Node) -> f32;
    /// Returns the cached height of the given node.
    fn height(&self, node: &Self::Node) -> f32;
    /// Returns the cached horizontal position of the given node.
    fn posx(&self, node: &Self::Node) -> f32;
    /// Returns the cached vertical position of the given node.
    fn posy(&self, node: &Self::Node) -> f32;

    /// Sets the cached position and size of the given node.
    fn set_bounds(&mut self, node: &Self::Node, posx: f32, posy: f32, width: f32, height: f32);
}

/// Helper trait for getting/setting node position/size in a direction agnostic way.
pub(crate) trait CacheExt: Cache {
    fn set_rect(
        &mut self,
        node: &Self::Node,
        parent_layout_type: LayoutType,
        main_pos: f32,
        cross_pos: f32,
        main: f32,
        cross: f32,
    ) {
        match parent_layout_type {
            LayoutType::Row => self.set_bounds(node, main_pos, cross_pos, main, cross),
            LayoutType::Column => self.set_bounds(node, cross_pos, main_pos, cross, main),
        }
    }
}

// Implement `CacheExt` for all types which implement `Cache`.
impl<C: Cache> CacheExt for C {}
