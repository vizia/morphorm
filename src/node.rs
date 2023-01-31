use crate::{LayoutType, PositionType, Units};

pub trait Node: Sized + Clone {
    type Store;
    type Tree;
    type ChildIter<'t>: Iterator<Item = &'t Self>
    where
        Self: 't;

    type CacheKey: std::fmt::Debug;

    fn key(&self) -> Self::CacheKey;

    /// Returns an iterator over the children of a node.
    fn children<'t>(&'t self, tree: &'t Self::Tree) -> Self::ChildIter<'t>;

    /// Returns the layout type of a node.
    fn layout_type(&self, store: &Self::Store) -> Option<LayoutType>;

    fn position_type(&self, store: &Self::Store) -> Option<PositionType>;

    /// Returns the size of the node on the main axis, as determined by the parent's layout type.
    fn width(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the size of the node on the cross axis, as determined by the parent's layout type.
    fn height(&self, store: &Self::Store) -> Option<Units>;

    fn left(&self, store: &Self::Store) -> Option<Units>;

    fn right(&self, store: &Self::Store) -> Option<Units>;

    fn top(&self, store: &Self::Store) -> Option<Units>;

    fn bottom(&self, store: &Self::Store) -> Option<Units>;

    fn content_size(&self, store: &Self::Store, cross_size: f32) -> Option<f32>;

    // Child Spacing

    fn child_left(&self, store: &Self::Store) -> Option<Units>;
    fn child_right(&self, store: &Self::Store) -> Option<Units>;
    fn child_top(&self, store: &Self::Store) -> Option<Units>;
    fn child_bottom(&self, store: &Self::Store) -> Option<Units>;

    fn row_between(&self, store: &Self::Store) -> Option<Units>;
    fn col_between(&self, store: &Self::Store) -> Option<Units>;

    fn min_width(&self, store: &Self::Store) -> Option<Units>;
    fn min_height(&self, store: &Self::Store) -> Option<Units>;
    fn max_width(&self, store: &Self::Store) -> Option<Units>;
    fn max_height(&self, store: &Self::Store) -> Option<Units>;
}

pub(crate) trait NodeExt: Node {

    fn main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.width(store),
            LayoutType::Column => self.height(store),
            _ => None,
        }
    }

    fn min_main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.min_width(store),
            LayoutType::Column => self.min_height(store),
            _ => None,
        }
    }

    fn max_main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.max_width(store),
            LayoutType::Column => self.max_height(store),
            _ => None,
        }
    }

    fn cross(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.height(store),
            LayoutType::Column => self.width(store),
            _ => None,
        }
    }

    fn min_cross(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.min_height(store),
            LayoutType::Column => self.min_width(store),
            _ => None,
        }
    }

    fn max_cross(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.max_height(store),
            LayoutType::Column => self.max_width(store),
            _ => None,
        }
    }

    fn main_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.left(store),
            LayoutType::Column => self.top(store),
            _ => None,
        }
    }

    fn main_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.right(store),
            LayoutType::Column => self.bottom(store),
            _ => None,
        }
    }

    fn cross_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.top(store),
            LayoutType::Column => self.left(store),
            _ => None,
        }
    }

    fn cross_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.bottom(store),
            LayoutType::Column => self.right(store),
            _ => None,
        }
    }

    fn child_main_before(
        &self,
        store: &Self::Store,
        parent_layout_type: LayoutType,
    ) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.child_left(store),
            LayoutType::Column => self.child_top(store),
            _ => None,
        }
    }

    fn child_main_after(
        &self,
        store: &Self::Store,
        parent_layout_type: LayoutType,
    ) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.child_right(store),
            LayoutType::Column => self.child_bottom(store),
            _ => None,
        }
    }

    fn child_cross_before(
        &self,
        store: &Self::Store,
        parent_layout_type: LayoutType,
    ) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.child_top(store),
            LayoutType::Column => self.child_left(store),
            _ => None,
        }
    }

    fn child_cross_after(
        &self,
        store: &Self::Store,
        parent_layout_type: LayoutType,
    ) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.child_bottom(store),
            LayoutType::Column => self.child_right(store),
            _ => None,
        }
    }

    fn main_between(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.col_between(store),
            LayoutType::Column => self.row_between(store),
            _ => None,
        }
    }

    // Currently unused until wrapping is implemented
    fn cross_between(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<Units> {
        match parent_layout_type {
            LayoutType::Row => self.row_between(store),
            LayoutType::Column => self.col_between(store),
            _ => None,
        }
    }
}

impl<N: Node> NodeExt for N {}
