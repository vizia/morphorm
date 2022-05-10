use crate::{Units, LayoutType};




pub trait Node<'t>: Sized 
where Self: 't,
{
    type Store;
    type Tree;
    type ChildIter: Iterator<Item = Self>;

    type CacheKey;

    fn key(&self) -> Self::CacheKey;

    fn children(&self, tree: &'t Self::Tree) -> Self::ChildIter;

    fn layout_type(&self, store: &Self::Store) -> Option<LayoutType>;

    fn width(&self, store: &Self::Store) -> Option<Units>;
    fn height(&self, store: &Self::Store) -> Option<Units>;

    // fn first_child<N: Node>(&self, store: &Self::Store, tree: &Self::Tree) -> Option<N>;

    // fn next_sibling<N: Node>(&self, store: &Self::Store, tree: &Self::Tree) -> Option<N>;
    // fn prev_sibling<N: Node>(&self, store: &Self::Store, tree: &Self::Tree) -> Option<N>;

}