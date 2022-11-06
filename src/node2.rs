use crate::{LayoutType, Units, PositionType};

pub trait Node<'t>: Sized + Clone
where
    Self: 't,
{
    type Store;
    type Tree;
    type ChildIter: Iterator<Item = &'t Self>;

    type CacheKey: std::fmt::Debug;

    fn key(&self) -> Self::CacheKey;

    /// Returns an iterator over the children of a node.
    fn children(&self, tree: &'t Self::Tree) -> Self::ChildIter;

    /// Returns the layout type of a node.
    fn layout_type(&self, store: &Self::Store) -> Option<LayoutType>;

    fn position_type(&self, store: &Self::Store) -> Option<PositionType>;

    /// Returns the size of the node on the main axis, as determined by the parent's layout type.
    fn main(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the size of the node on the cross axis, as determined by the parent's layout type.
    fn cross(&self, store: &Self::Store) -> Option<Units>;

    fn main_before(&self, store: &Self::Store) -> Option<Units>;

    fn main_after(&self, store: &Self::Store) -> Option<Units>;

    fn cross_before(&self, store: &Self::Store) -> Option<Units>;

    fn cross_after(&self, store: &Self::Store) -> Option<Units>;

    fn content_size(&self, store: &Self::Store, cross_size: f32) -> Option<f32>;

    fn child_main_before(&self, store: &Self::Store) -> Option<Units>;
    fn child_main_after(&self, store: &Self::Store) -> Option<Units>;
    fn child_cross_before(&self, store: &Self::Store) -> Option<Units>;
    fn child_cross_after(&self, store: &Self::Store) -> Option<Units>;
}
