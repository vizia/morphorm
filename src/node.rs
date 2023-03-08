use crate::{LayoutType, PositionType, Units};

pub trait Node: Sized + Clone {
    /// A type representing a store where layout properties are stored.
    type Store;
    /// A type representing a tree structure where the children of the node are stored.
    type Tree;
    /// An type reresenting an iterator over the children of the node.
    type ChildIter<'t>: Iterator<Item = &'t Self>
    where
        Self: 't;

    /// A type representing a key to store and retrieve values from the [`Cache`](crate::Cache).
    /// This type must be the same as the [`CacheKey`](crate::Cache::CacheKey) associated type on the [`Cache`](crate::Cache) trait.
    type CacheKey;

    /// Returns a key which can be used to set/get computed layout data fron the [`cache`](crate::Cache).
    fn key(&self) -> Self::CacheKey;

    /// Returns an iterator over the children of the node.
    fn children<'t>(&'t self, tree: &'t Self::Tree) -> Self::ChildIter<'t>;

    /// Returns the layout type of the node.
    fn layout_type(&self, store: &Self::Store) -> Option<LayoutType>;

    /// Returns the position type of the node.
    fn position_type(&self, store: &Self::Store) -> Option<PositionType>;

    /// Returns the desired width of the node.
    fn width(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired height of the node.
    fn height(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired left-side space of the node.
    fn left(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired right-side space of the node.
    fn right(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired top-side space of the node.
    fn top(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired bottom-side space of the node.
    fn bottom(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired main-axis size given a computed cross-axis size.
    fn content_main(&self, store: &Self::Store, cross: f32) -> Option<f32>;

    /// Returns the desired cross-axis size given a computed main-axis size.
    fn content_cross(&self, store: &Self::Store, main: f32) -> Option<f32>;

    // Child Spacing

    /// Returns the desired left-side child-space of the node.
    fn child_left(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired left-side child-space of the node.
    fn child_right(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired left-side child-space of the node.
    fn child_top(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired left-side child-space of the node.
    fn child_bottom(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired space to applied between the children of the node on the vertical axis.
    fn row_between(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired space to be applied between the children of the node on the horizontal axis.
    fn col_between(&self, store: &Self::Store) -> Option<Units>;

    fn min_width(&self, store: &Self::Store) -> Option<Units>;
    fn min_height(&self, store: &Self::Store) -> Option<Units>;
    fn max_width(&self, store: &Self::Store) -> Option<Units>;
    fn max_height(&self, store: &Self::Store) -> Option<Units>;

    fn min_left(&self, store: &Self::Store) -> Option<Units>;
    fn min_right(&self, store: &Self::Store) -> Option<Units>;
    fn min_top(&self, store: &Self::Store) -> Option<Units>;
    fn min_bottom(&self, store: &Self::Store) -> Option<Units>;

    fn max_left(&self, store: &Self::Store) -> Option<Units>;
    fn max_right(&self, store: &Self::Store) -> Option<Units>;
    fn max_top(&self, store: &Self::Store) -> Option<Units>;
    fn max_bottom(&self, store: &Self::Store) -> Option<Units>;
}

/// Helper trait for converting layout properties into a direction-agnostic value.
pub(crate) trait NodeExt: Node {
    fn main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        match parent_layout_type {
            LayoutType::Row => self.width(store).unwrap_or(Units::Stretch(1.0)),
            LayoutType::Column => self.height(store).unwrap_or(Units::Stretch(1.0)),
        }
    }

    fn min_main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.min_width(store), |store| self.min_height(store))
    }

    fn max_main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.max_width(store), |store| self.max_height(store))
    }

    fn cross(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        match parent_layout_type {
            LayoutType::Row => self.height(store).unwrap_or(Units::Stretch(1.0)),
            LayoutType::Column => self.width(store).unwrap_or(Units::Stretch(1.0)),
        }
    }

    fn min_cross(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.min_height(store), |store| self.min_width(store))
    }

    fn max_cross(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.max_height(store), |store| self.max_width(store))
    }

    fn main_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.left(store), |store| self.top(store))
    }

    fn main_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.right(store), |store| self.bottom(store))
    }

    fn cross_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.top(store), |store| self.left(store))
    }

    fn cross_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.bottom(store), |store| self.right(store))
    }

    fn child_main_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.child_left(store), |store| self.child_top(store))
    }

    fn child_main_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.child_right(store), |store| self.child_bottom(store))
    }

    fn child_cross_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.child_top(store), |store| self.child_left(store))
    }

    fn child_cross_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.child_bottom(store), |store| self.child_right(store))
    }

    fn main_between(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.col_between(store), |store| self.row_between(store))
    }

    // Currently unused until wrapping is implemented
    fn cross_between(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.row_between(store), |store| self.col_between(store))
    }

    fn min_main_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.min_left(store), |store| self.min_top(store))
    }

    fn max_main_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.max_left(store), |store| self.max_top(store))
    }

    fn min_main_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.min_right(store), |store| self.min_bottom(store))
    }

    fn max_main_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.max_right(store), |store| self.max_bottom(store))
    }

    fn min_cross_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.min_top(store), |store| self.min_left(store))
    }

    fn max_cross_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.max_top(store), |store| self.max_left(store))
    }

    fn min_cross_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.min_bottom(store), |store| self.min_right(store))
    }

    fn max_cross_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.max_bottom(store), |store| self.max_right(store))
    }
}

impl<N: Node> NodeExt for N {}
