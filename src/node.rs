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
/// a reference to which is passed to the [`children`](crate::Node::children) method, which returns an iterator on the children of the node,
/// the type of which is specified by the `ChildIter` associated type.
pub trait Node: Sized {
    /// A type representing a store where layout properties can be stored.
    type Store;
    /// A type representing a tree structure where the children of the node can be stored.
    type Tree;
    /// An type representing an iterator over the children of the node.
    type ChildIter<'t>: Iterator<Item = &'t Self>
    where
        Self: 't;
    /// A type representing a key to store and retrieve values from the [`Cache`].
    type CacheKey: std::fmt::Debug;
    /// A type representing a context which can be used to save/load state when computing [content size](crate::Node::content_size).
    /// For example, a `TextContext` which could be used to measure (and cache) the size of text, which could
    /// then be used to size an `Auto` layout node using content size.
    type SubLayout<'a>;

    /// Performs layout on the given node returning its computed size.
    ///
    /// The algorithm recurses down the tree, in depth-first order, and performs
    /// layout on every node starting from the input `node`.
    ///
    /// # Arguments
    ///
    /// * `cache` - A mutable reference to the [`Cache`].
    /// * `tree` - A mutable reference to the [`Tree`](crate::Node::Tree).
    /// * `store` - A mutable reference to the [`Store`](crate::Node::Store).
    /// * `sublayout` - A mutable reference to the [`SubLayout`](crate::Node::SubLayout) context.
    ///
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

    /// Returns the alignment of the node.
    fn alignment(&self, store: &Self::Store) -> Option<Alignment>;

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

    /// Returns the width and height of the node if its desired width and/or desired height are auto and the node has no children.
    /// This can be used to size the node based on visual content (such as text), or to apply an aspect ratio size constraint.
    fn content_size(
        &self,
        store: &Self::Store,
        sublayout: &mut Self::SubLayout<'_>,
        parent_width: Option<f32>,
        parent_height: Option<f32>,
    ) -> Option<(f32, f32)>;

    /// Returns the desired left-side child-space of the node.
    fn padding_left(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired left-side child-space of the node.
    fn padding_right(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired left-side child-space of the node.
    fn padding_top(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired left-side child-space of the node.
    fn padding_bottom(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired space to applied between the children of the node on the vertical axis.
    fn vertical_gap(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired space to be applied between the children of the node on the horizontal axis.
    fn horizontal_gap(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired minimum space to applied between the children of the node on the vertical axis.
    fn min_vertical_gap(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired minimum space to be applied between the children of the node on the horizontal axis.
    fn min_horizontal_gap(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired maximum space to applied between the children of the node on the vertical axis.
    fn max_vertical_gap(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the desired maximum space to be applied between the children of the node on the horizontal axis.
    fn max_horizontal_gap(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the minimum width of the node.
    fn min_width(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the minimum height of the node.
    fn min_height(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the maximum width of the node.
    fn max_width(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the maximum height of the node.
    fn max_height(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the left-side border width of the node.
    fn border_left(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the right-side border width of the node.
    fn border_right(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the top-side border width of the node.
    fn border_top(&self, store: &Self::Store) -> Option<Units>;

    /// Returns the bottom-side border width of the node.
    fn border_bottom(&self, store: &Self::Store) -> Option<Units>;

    fn vertical_scroll(&self, store: &Self::Store) -> Option<f32>;

    fn horizontal_scroll(&self, store: &Self::Store) -> Option<f32>;
}

/// Helper trait used internally for converting layout properties into a direction-agnostic value.
pub(crate) trait NodeExt: Node {
    fn main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        match parent_layout_type {
            LayoutType::Row => self.width(store).unwrap_or(Units::Stretch(1.0)),
            LayoutType::Column => self.height(store).unwrap_or(Units::Stretch(1.0)),
        }
    }

    fn min_main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap_default(
            store,
            |store| self.min_width(store),
            |store| self.min_height(store),
            Units::Pixels(0.0),
        )
    }

    fn max_main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap_default(
            store,
            |store| self.max_width(store),
            |store| self.max_height(store),
            Units::Pixels(f32::MAX),
        )
    }

    fn cross(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        match parent_layout_type {
            LayoutType::Row => self.height(store).unwrap_or(Units::Stretch(1.0)),
            LayoutType::Column => self.width(store).unwrap_or(Units::Stretch(1.0)),
        }
    }

    fn min_cross(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap_default(
            store,
            |store| self.min_height(store),
            |store| self.min_width(store),
            Units::Pixels(0.0),
        )
    }

    fn max_cross(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap_default(
            store,
            |store| self.max_height(store),
            |store| self.max_width(store),
            Units::Pixels(f32::MAX),
        )
    }

    fn main_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap_default(store, |store| self.left(store), |store| self.top(store), Units::Auto)
    }

    fn main_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap_default(
            store,
            |store| self.right(store),
            |store| self.bottom(store),
            Units::Auto,
        )
    }

    fn cross_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap_default(store, |store| self.top(store), |store| self.left(store), Units::Auto)
    }

    fn cross_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap_default(
            store,
            |store| self.bottom(store),
            |store| self.right(store),
            Units::Auto,
        )
    }

    fn padding_main_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.padding_left(store), |store| self.padding_top(store))
    }

    fn padding_main_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.padding_right(store), |store| self.padding_bottom(store))
    }

    fn padding_cross_before(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.padding_top(store), |store| self.padding_left(store))
    }

    fn padding_cross_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.padding_bottom(store), |store| self.padding_right(store))
    }

    fn main_between(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.horizontal_gap(store), |store| self.vertical_gap(store))
    }

    fn min_main_between(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(
            store,
            |store| self.min_horizontal_gap(store),
            |store| self.min_vertical_gap(store),
        )
    }

    fn max_main_between(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(
            store,
            |store| self.max_horizontal_gap(store),
            |store| self.max_vertical_gap(store),
        )
    }

    // Currently unused until wrapping is implemented
    #[allow(dead_code)]
    fn cross_between(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.vertical_gap(store), |store| self.horizontal_gap(store))
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

    fn cross_scroll(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<f32> {
        parent_layout_type.select(store, |store| self.vertical_scroll(store), |store| self.horizontal_scroll(store))
    }

    fn main_scroll(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Option<f32> {
        parent_layout_type.select(store, |store| self.horizontal_scroll(store), |store| self.vertical_scroll(store))
    }
}

// Implement `NodeExt` for all types which implement `Node`.
impl<N: Node> NodeExt for N {}
