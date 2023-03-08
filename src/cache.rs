use crate::{LayoutType, Node};

pub trait Cache {
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

pub(crate) trait CacheExt: Cache {
    fn main(&self, node: &Self::Node, parent_layout_type: LayoutType) -> f32 {
        parent_layout_type.select(node, |node| self.width(node), |node| self.height(node))
    }

    fn cross(&self, node: &Self::Node, parent_layout_type: LayoutType) -> f32 {
        parent_layout_type.select(node, |node| self.height(node), |node| self.width(node))
    }

    fn main_pos(&self, node: &Self::Node, parent_layout_type: LayoutType) -> f32 {
        parent_layout_type.select(node, |node| self.posx(node), |node| self.posy(node))
    }

    fn cross_pos(&self, node: &Self::Node, parent_layout_type: LayoutType) -> f32 {
        parent_layout_type.select(node, |node| self.posy(node), |node| self.posx(node))
    }

    fn set_main(&mut self, node: &Self::Node, parent_layout_type: LayoutType, main: f32) {
        match parent_layout_type {
            LayoutType::Row => self.set_width(node, main),
            LayoutType::Column => self.set_height(node, main),
        }
    }

    fn set_cross(&mut self, node: &Self::Node, parent_layout_type: LayoutType, cross: f32) {
        match parent_layout_type {
            LayoutType::Row => self.set_height(node, cross),
            LayoutType::Column => self.set_width(node, cross),
        }
    }

    fn set_main_pos(&mut self, node: &Self::Node, parent_layout_type: LayoutType, main_pos: f32) {
        match parent_layout_type {
            LayoutType::Row => self.set_posx(node, main_pos),
            LayoutType::Column => self.set_posy(node, main_pos),
        }
    }

    fn set_cross_pos(&mut self, node: &Self::Node, parent_layout_type: LayoutType, cross_pos: f32) {
        match parent_layout_type {
            LayoutType::Row => self.set_posy(node, cross_pos),
            LayoutType::Column => self.set_posx(node, cross_pos),
        }
    }
}

impl<C: Cache> CacheExt for C {}
