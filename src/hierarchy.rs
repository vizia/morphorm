
use crate::node::Node;

/// Describes a visual tree of nodes which can be layed out
pub trait Hierarchy<'a> {
    /// A type respresenting a node in the visual tree
    type Item: 'a + Node;
    /// A type respresenting an iterator that walks up the visual tree 
    type UpIter: Iterator<Item = Self::Item>;
    /// A type representing an iterator that walks down the visual tree
    type DownIter: Iterator<Item = Self::Item>;
    /// A type representing an iterator which iterates through the children of a specified node
    type ChildIter: Iterator<Item = &'a Self::Item>;

    /// Returns an iterator which walks up the hierarchy
    fn up_iter(&self, store: &<Self::Item as Node>::Data) -> Self::UpIter;

    /// Returns an iterator which walks down the hierarchy
    fn down_iter(&self, store: &<Self::Item as Node>::Data) -> Self::DownIter;

    /// Returns an iterator over the child nodes of a specified node
    fn child_iter(&'a self, node: &Self::Item) -> Self::ChildIter;

    /// Get the parent node of the specified node
    fn parent(&self, node: &Self::Item) -> Option<&Self::Item>;

    /// Returns true if the specified node is the first child of its parent
    fn is_first_child(&self, node: &Self::Item) -> bool;

    /// Returns true if the specified node is the last child of its parent
    fn is_last_child(&self, node: &Self::Item) -> bool;
}
