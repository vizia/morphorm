use crate::{Cache, Node, Size, IncrementalInput, IncrementalResult, ParentLayoutInput, LayoutType};

/// Extension trait for nodes to support incremental layout.
///
/// This trait provides methods for performing incremental layout passes on a subtree,
/// with automatic constraint signature caching and dependency tracking.
pub trait LayoutExt: Node {
    /// Perform a standard full-tree layout pass starting from this node.
    ///
    /// This is the traditional full relayout operation.
    fn layout_full<C: Cache<Node = Self>>(
        &self,
        parent_layout_type: LayoutType,
        parent_main: f32,
        parent_cross: f32,
        cache: &mut C,
        tree: &Self::Tree,
        store: &Self::Store,
        sublayout: &mut Self::SubLayout<'_>,
    ) -> Size
    where
        Self::CacheKey: Clone,
    {
        crate::layout::layout_full(self, parent_layout_type, parent_main, parent_cross, cache, tree, store, sublayout)
    }

    /// Perform an incremental layout pass on this node and its descendants.
    ///
    /// This is the recommended method for updating layout when only a portion of the tree
    /// has changed. It uses constraint signature caching to skip stable nodes and
    /// detects when changes propagate outside the local closure.
    ///
    /// # Arguments
    ///
    /// * `input` - Specifies dirty nodes, parent constraints, and escalation boundary.
    /// * `cache` - Mutable reference to the layout cache.
    /// * `tree` - Tree structure containing node relationships.
    /// * `store` - Store containing layout properties.
    /// * `sublayout` - Context for measuring content size.
    ///
    /// # Returns
    ///
    /// - `IncrementalResult::Converged` - Layout is stable within the scope.
    /// - `IncrementalResult::EscapedScope` - Changes affected nodes outside the boundary.
    /// - `IncrementalResult::Diverged` - Layout failed to converge.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut dirty_nodes = HashSet::new();
    /// dirty_nodes.insert(node_key);
    ///
    /// let input = IncrementalInput {
    ///     root: root_key,
    ///     parent_layout_input: ParentLayoutInput {
    ///         parent_main: 800.0,
    ///         parent_cross: 600.0,
    ///         parent_layout_type: LayoutType::Column,
    ///     },
    ///     dirty_nodes,
    ///     escalation_boundary: Some(subtree_boundary),
    /// };
    ///
    /// match root_node.layout_incremental(&input, &mut cache, &tree, &store, &mut sublayout) {
    ///     IncrementalResult::Converged => println!("Layout stable"),
    ///     IncrementalResult::EscapedScope => println!("Need wider pass"),
    ///     IncrementalResult::Diverged => println!("Need full relayout"),
    /// }
    /// ```
    fn layout_incremental<C: Cache<Node = Self>>(
        &self,
        input: &IncrementalInput<Self::CacheKey>,
        cache: &mut C,
        tree: &Self::Tree,
        store: &Self::Store,
        sublayout: &mut Self::SubLayout<'_>,
    ) -> IncrementalResult
    where
        Self::CacheKey: Clone,
    {
        crate::layout::layout_incremental(self, input, cache, tree, store, sublayout)
    }

    /// Perform a subtree layout with explicit parent constraint input.
    ///
    /// This is useful for incremental updates where you know the parent constraints
    /// that should apply to this node. It's more explicit than the standard `layout` method.
    ///
    /// # Arguments
    ///
    /// * `parent_layout_input` - Parent-provided constraints and layout type.
    /// * `cache` - Mutable reference to the layout cache.
    /// * `tree` - Tree structure containing node relationships.
    /// * `store` - Store containing layout properties.
    /// * `sublayout` - Context for measuring content size.
    fn layout_subtree<C: Cache<Node = Self>>(
        &self,
        parent_layout_input: ParentLayoutInput,
        cache: &mut C,
        tree: &Self::Tree,
        store: &Self::Store,
        sublayout: &mut Self::SubLayout<'_>,
    ) -> Size
    where
        Self::CacheKey: Clone,
    {
        crate::layout::layout_subtree(
            self,
            parent_layout_input.parent_layout_type,
            parent_layout_input.parent_main,
            parent_layout_input.parent_cross,
            cache,
            tree,
            store,
            sublayout,
        )
    }
}

// Implement the extension trait for all nodes.
impl<N: Node> LayoutExt for N {}
