use crate::{layout, types::*, Cache, LayoutWrap};

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

    /// Performs layout on the given node.
    ///
    /// The algorithm recurses down the tree in depth-first order and performs
    /// layout on every node in the restarted subtree. During incremental relayout,
    /// the input node is treated as dirty and layout may restart from an ancestor
    /// selected by [`NodeExt::find_relayout_root`]. Calling this on the tree root
    /// still performs a full layout pass.
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
    ) {
        // Incremental layout: `self` is the node which has been marked as dirty. Rather than
        // always laying out from `self`, find the best ancestor to restart layout from based on
        // whether the change can affect the ancestor. Layout is then performed from that ancestor,
        // recursing through all of its descendants (no unchanged descendants are skipped).
        //
        // When `self` is the root of the tree the returned ancestor is `self`, so calling
        // `root.layout(..)` performs a full layout pass exactly as before.
        let root = self.find_relayout_root(tree, store);

        // Determine the size of the restart root.
        //
        // A non-root ancestor keeps its previously computed (cached) size. By construction this
        // ancestor is sized in `Pixels` or `Stretch`, so its size is stable under the change and is
        // reproduced exactly by the layout algorithm when fed the cached size.
        //
        // The tree root (which has no parent) is sized from its own properties, preserving the
        // behavior of a full layout pass and working on the first pass when the cache is empty.
        let (width, height) = if root.parent(tree).is_some() {
            (cache.width(root), cache.height(root))
        } else {
            let width = root.width(store).unwrap_or(Units::Pixels(0.0)).to_px(0.0, 0.0);
            let height = root.height(store).unwrap_or(Units::Pixels(0.0)).to_px(0.0, 0.0);
            (width, height)
        };

        cache.set_bounds(root, cache.posx(root), cache.posy(root), width, height);

        // Use the node's layout type instead of hardcoding Column
        let layout_type = root.layout_type(store).unwrap_or_default();
        let (parent_main, parent_cross) = match layout_type {
            LayoutType::Row | LayoutType::Overlay | LayoutType::Grid => (width, height), // Row/Overlay/Grid: main=width, cross=height
            LayoutType::Column => (height, width), // Column: main=height, cross=width
        };

        layout(root, layout_type, parent_main, parent_cross, cache, tree, store, sublayout);
    }

    /// Returns a key which can be used to set/get computed layout data from the [`cache`](crate::Cache).
    fn key(&self) -> Self::CacheKey;

    /// Returns an iterator over the children of the node.
    fn children<'t>(&'t self, tree: &'t Self::Tree) -> Self::ChildIter<'t>;

    /// Returns an optional reference to the parent of the node.
    fn parent<'t>(&'t self, tree: &'t Self::Tree) -> Option<&'t Self>;

    /// Returns a boolean representing whether the node is visible to layout.
    fn visible(&self, store: &Self::Store) -> bool;

    /// Returns the layout type of the node.
    fn layout_type(&self, store: &Self::Store) -> Option<LayoutType>;

    /// Returns the position type of the node.
    fn position_type(&self, store: &Self::Store) -> Option<PositionType>;

    /// Returns the inline direction used for horizontal positioning semantics.
    fn direction(&self, _store: &Self::Store) -> Option<Direction> {
        None
    }

    /// Returns whether children wrap to a new line when they overflow the main axis.
    ///
    /// Defaults to `None` which is treated as [`LayoutWrap::NoWrap`].
    fn wrap(&self, _store: &Self::Store) -> Option<LayoutWrap> {
        None
    }

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

    fn grid_columns(&self, store: &Self::Store) -> Option<Vec<Units>>;

    fn grid_rows(&self, store: &Self::Store) -> Option<Vec<Units>>;

    fn column_start(&self, store: &Self::Store) -> Option<usize>;

    fn row_start(&self, store: &Self::Store) -> Option<usize>;

    fn column_span(&self, store: &Self::Store) -> Option<usize>;

    fn row_span(&self, store: &Self::Store) -> Option<usize>;
}

/// Helper trait used internally for converting layout properties into a direction-agnostic value.
pub(crate) trait NodeExt: Node {
    fn main(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        match parent_layout_type {
            LayoutType::Row | LayoutType::Overlay | LayoutType::Grid => {
                self.width(store).unwrap_or(Units::Stretch(1.0))
            }
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
            LayoutType::Row | LayoutType::Overlay | LayoutType::Grid => {
                self.height(store).unwrap_or(Units::Stretch(1.0))
            }
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
        if parent_layout_type == LayoutType::Row && self.direction(store).unwrap_or_default() == Direction::RightToLeft
        {
            self.padding_right(store).unwrap_or_default()
        } else {
            parent_layout_type.select_unwrap(store, |store| self.padding_left(store), |store| self.padding_top(store))
        }
    }

    fn padding_main_after(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        if parent_layout_type == LayoutType::Row && self.direction(store).unwrap_or_default() == Direction::RightToLeft
        {
            self.padding_left(store).unwrap_or_default()
        } else {
            parent_layout_type.select_unwrap(
                store,
                |store| self.padding_right(store),
                |store| self.padding_bottom(store),
            )
        }
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

    fn cross_between(&self, store: &Self::Store, parent_layout_type: LayoutType) -> Units {
        parent_layout_type.select_unwrap(store, |store| self.vertical_gap(store), |store| self.horizontal_gap(store))
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
            LayoutType::Row | LayoutType::Overlay | LayoutType::Grid => {
                self.content_size(store, sublayout, parent_main, parent_cross)
            }

            LayoutType::Column => {
                self.content_size(store, sublayout, parent_cross, parent_main).map(|(width, height)| (height, width))
            }
        }
    }

    /// Returns whether the node can be used as a restart point for incremental layout.
    ///
    /// A node is a valid restart point when its size is both *stable* under changes to its
    /// descendants and *reproducible* from its cached size:
    /// - [`Units::Pixels`] is fixed regardless of children or parent.
    /// - [`Units::Stretch`] is determined by the parent's allocation (which is unaffected by the
    ///   node's own descendants) and is reproduced by the layout algorithm when fed the cached size.
    ///
    /// [`Units::Auto`] sizes to fit children (so a descendant change can change the node's size,
    /// affecting its parent) and [`Units::Percentage`] cannot be reproduced from the cached size
    /// alone, so neither is a valid restart point.
    ///
    /// A missing width/height is treated as [`Units::Stretch`], matching the default used by the
    /// layout algorithm ([`main`](NodeExt::main)/[`cross`](NodeExt::cross)).
    fn is_restartable(&self, store: &Self::Store) -> bool {
        fn stable(units: Units) -> bool {
            units.is_pixels() || units.is_stretch()
        }

        let width = self.width(store).unwrap_or(Units::Stretch(1.0));
        let height = self.height(store).unwrap_or(Units::Stretch(1.0));
        let min_width = self.min_width(store).unwrap_or(Units::Pixels(0.0));
        let max_width = self.max_width(store).unwrap_or(Units::Pixels(f32::MAX));
        let min_height = self.min_height(store).unwrap_or(Units::Pixels(0.0));
        let max_height = self.max_height(store).unwrap_or(Units::Pixels(f32::MAX));

        stable(width)
            && stable(height)
            && stable(min_width)
            && stable(max_width)
            && stable(min_height)
            && stable(max_height)
    }

    /// Finds the best ancestor to restart layout from for a node which has been marked as dirty.
    ///
    /// Layout must restart from at least the node's parent, since the parent determines the size and
    /// position of the node relative to its siblings. From there the search walks up the tree while
    /// each ancestor's size could still affect its own parent (i.e. it is not
    /// [restartable](NodeExt::is_restartable)), stopping at the first restartable ancestor or at the
    /// root of the tree.
    ///
    /// An [absolutely-positioned](PositionType::Absolute) ancestor is taken out of its parent's flow,
    /// so it does not affect the parent's size. However, its own position can depend on its size
    /// (e.g. right/bottom anchoring), so relayout should restart from that absolute ancestor's parent
    /// to recompute the absolute position.
    ///
    /// If the node is the root of the tree it is returned unchanged, so a call on the root performs
    /// a full layout pass.
    fn find_relayout_root<'t>(&'t self, tree: &'t Self::Tree, store: &Self::Store) -> &'t Self {
        // Always restart from at least the parent of the dirty node.
        let mut root = match self.parent(tree) {
            Some(parent) => parent,
            None => return self,
        };

        // If the dirty node is itself absolutely positioned it is out of its parent's flow, so a
        // change to it (including being added or removed) cannot alter the parent's size. Restart
        // from the parent — which positions the absolute node relative to its cached bounds —
        // without walking any further up the tree.
        if self.position_type(store).unwrap_or_default() == PositionType::Absolute {
            return root;
        }

        // Walk up while the current ancestor's size could affect its own parent.
        while let Some(parent) = root.parent(tree) {
            // Absolutely-positioned nodes don't affect their parent's size, but their own
            // position can depend on their size (e.g. right/bottom anchoring). Restart at the
            // parent so absolute positioning is recomputed.
            if root.position_type(store).unwrap_or_default() == PositionType::Absolute {
                return parent;
            }
            if root.is_restartable(store) {
                break;
            }
            root = parent;
        }

        root
    }
}

// Implement `NodeExt` for all types which implement `Node`.
impl<N: Node> NodeExt for N {}
