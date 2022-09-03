use crate::{LayoutType, Units};

pub trait Node<'t>: Sized + Clone
where
    Self: 't,
{
    type Store;
    type Tree;
    type ChildIter: Iterator<Item = &'t Self>;

    type CacheKey: std::fmt::Debug;

    fn key(&self) -> Self::CacheKey;

    fn children(&self, tree: &'t Self::Tree) -> Self::ChildIter;

    fn layout_type(&self, store: &Self::Store) -> Option<LayoutType>;

    fn main(&self, store: &Self::Store) -> Option<Units>;

    fn cross(&self, store: &Self::Store) -> Option<Units>;

    fn main_before(&self, store: &Self::Store) -> Option<Units>;

    fn content_size(&self, store: &Self::Store, cross_size: f32) -> Option<f32>;

    // fn first_child<N: Node>(&self, store: &Self::Store, tree: &Self::Tree) -> Option<N>;

    // fn next_sibling<N: Node>(&self, store: &Self::Store, tree: &Self::Tree) -> Option<N>;
    // fn prev_sibling<N: Node>(&self, store: &Self::Store, tree: &Self::Tree) -> Option<N>;
}
