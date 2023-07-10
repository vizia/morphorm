use crate::{layout, types::*, Cache};

/// A `Node` represents a layout element which can be sized and positioned based on
/// a number of layout properties.
///
/// The getter methods in this trait allow for the layout function to retrieve the
/// layout properties of the node. The `Node` trait allows for its layout properties to optionally
/// be stored externally from the node type itself by providing a `Store` associated type, a reference to
/// which is passed to the layout property methods.
///
/// Similarly, the children of the node can be optionally stored externally using the `Tree` associated type,
/// a reference to which is passed to the `children` method, which returns an iterator on the children of the node,
/// the type of which is specified by the `ChildIter` associated type.
pub trait Node: Sized + Clone {
    /// A type representing a store where layout properties can be stored.
    type Store;
    /// A type representing a tree structure where the children of the node can be stored.
    type Tree;
    /// An type representing an iterator over the children of the node.
    type ChildIter<'t>: Iterator<Item = &'t Self>
    where
        Self: 't;

    /// A type representing a key to store and retrieve values from the [`Cache`](crate::Cache).
    type CacheKey;

    type SubLayout<'a>;

    fn layout<C: Cache<Node = Self>>(
        &self,
        cache: &mut C,
        tree: &Self::Tree,
        store: &Self::Store,
        sublayout: &mut Self::SubLayout<'_>,
    ) -> Size {
        let width = self
            .width(store)
            .map(|w| match w {
                Units::Pixels(px) => px,
                _ => panic!("Root node must have fixed size."),
            })
            .expect("Failed to get width for node");

        let height = self
            .height(store)
            .map(|w| match w {
                Units::Pixels(px) => px,
                _ => panic!("Root node must have fixed size."),
            })
            .expect("Failed to get height for node");

        cache.set_bounds(self, cache.posx(self), cache.posy(self), width, height);

        layout(self, LayoutType::Column, height, width, cache, tree, store, sublayout)
    }

    /// Returns a key which can be used to set/get computed layout data from the [`cache`](crate::Cache).
    fn key(&self) -> Self::CacheKey;

    /// Returns an iterator over the children of the node.
    fn children<'t>(&'t self, tree: &'t Self::Tree) -> Self::ChildIter<'t>;

    /// Returns a boolean representing whether the node is visible to layout.
    fn visible(&self, store: &Self::Store) -> bool;

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

    /// Returns the width and height of the node if its desired width and/or desired height are auto.
    fn content_size(
        &self,
        store: &Self::Store,
        sublayout: &mut Self::SubLayout<'_>,
        parent_width: Option<f32>,
        parent_height: Option<f32>,
    ) -> Option<(f32, f32)>;

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

    /// Returns the minimum width of the node.
    fn min_width(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the minimum height of the node.
    fn min_height(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the maximum width of the node.
    fn max_width(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the maximum height of the node.
    fn max_height(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the minimum left-side space of the node.
    fn min_left(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the minimum right-side space of the node.
    fn min_right(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the minimum top-side space of the node.
    fn min_top(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the minimum bottom-side space of the node.
    fn min_bottom(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the maximum left-side space of the node.
    fn max_left(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the maximum right-side space of the node.
    fn max_right(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the maximum top-side space of the node.
    fn max_top(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the maximum bottom-side space of the node.
    fn max_bottom(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the left-side border width of the node.
    fn border_left(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the right-side border width of the node.
    fn border_right(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the top-side border width of the node.
    fn border_top(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the bottom-side border width of the node.
    fn border_bottom(&self, store: &Self::Store) -> Option<Units>;
}

/// Helper trait for converting layout properties into a direction-agnostic value.
pub(crate) trait NodeExt: Node {
    fn main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        match parent_layout_type {
            LayoutType::Row => self.width(store).unwrap_or(Units::Auto),
            LayoutType::Column => self.height(store).unwrap_or(Units::Auto),
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

    fn border_main_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.border_left(store), |store| self.border_top(store))
    }

    fn border_main_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.border_right(store), |store| self.border_bottom(store))
    }

    fn border_cross_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.border_top(store), |store| self.border_left(store))
    }

    fn border_cross_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.border_bottom(store), |store| self.border_right(store))
    }

    fn content_sizing(
        &self,
        store: &Self::Store,
        sublayout: &mut Self::SubLayout<'_>,
        parent_layout_type: LayoutType,
        parent_main: Option<f32>,
        parent_cross: Option<f32>,
    ) -> Option<(f32, f32)> {
        match parent_layout_type {
            LayoutType::Row => self.content_size(store, sublayout, parent_main, parent_cross),

            LayoutType::Column => {
                self.content_size(store, sublayout, parent_cross, parent_main).map(|(width, height)| (height, width))
            }
        }
    }
}

impl<N: Node> NodeExt for N {}
