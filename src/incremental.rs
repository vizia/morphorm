use crate::{LayoutType, Node, NodeExt};
use std::collections::{HashMap, HashSet};

/// A compact signature of constraint inputs that produced a layout.
///
/// This prevents recomputation of nodes whose inputs haven't changed,
/// while still catching cases where ancestor constraints or sibling flow changed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstraintSignature {
    /// Parent-provided main-axis constraint (resolved width or height).
    pub parent_main: u32,
    /// Parent-provided cross-axis constraint (resolved width or height).
    pub parent_cross: u32,
    /// Parent's layout type (determines axis interpretation).
    pub parent_layout_type: LayoutType,
    /// Hash of intrinsic content size if present.
    pub content_size_hash: u32,
    /// Combined hash of min/max constraints on node.
    pub constraint_hash: u32,
}

impl ConstraintSignature {
    /// Compute a signature from layout inputs.
    pub fn compute<N: Node>(
        node: &N,
        parent_layout_type: LayoutType,
        parent_main: f32,
        parent_cross: f32,
        store: &N::Store,
    ) -> Self {
        // Quantize floats to u32 to handle floating-point imprecision.
        let parent_main = (parent_main * 1000.0) as u32;
        let parent_cross = (parent_cross * 1000.0) as u32;

        // Compute a simple hash of constraint properties.
        let min_main = node
            .min_main(store, parent_layout_type)
            .to_px(parent_main as f32 / 1000.0, -f32::MAX);
        let max_main = node
            .max_main(store, parent_layout_type)
            .to_px(parent_main as f32 / 1000.0, f32::MAX);
        let min_cross = node
            .min_cross(store, parent_layout_type)
            .to_px(parent_cross as f32 / 1000.0, -f32::MAX);
        let max_cross = node
            .max_cross(store, parent_layout_type)
            .to_px(parent_cross as f32 / 1000.0, f32::MAX);

        let constraint_hash = hash_floats(&[min_main, max_main, min_cross, max_cross]);
        let content_size_hash = 0; // Placeholder; content_size is stateful and harder to hash.

        ConstraintSignature {
            parent_main,
            parent_cross,
            parent_layout_type,
            content_size_hash,
            constraint_hash,
        }
    }
}

/// Tracks node dependencies and invalidation flow.
///
/// Used to understand which nodes depend on:
/// - parent size
/// - sibling sizes/order
/// - intrinsic content
/// - min/max constraints
/// - percent units
#[derive(Debug, Clone)]
pub struct DependencyGraph<K: std::hash::Hash + Eq + Clone> {
    /// Nodes that depend on parent size (e.g., percentage units or stretch).
    pub parent_size_deps: HashSet<K>,
    /// Nodes that depend on sibling flow (e.g., layout before this node).
    pub sibling_flow_deps: HashSet<K>,
    /// Nodes that depend on intrinsic content size.
    pub content_deps: HashSet<K>,
    /// Upstream ancestors that may need recompute (e.g., auto-sizing parents).
    pub upstream_deps: HashMap<K, K>, // child -> parent
    /// Downstream descendants affected by this node's change.
    pub downstream_deps: HashMap<K, Vec<K>>, // node -> children
}

impl<K: std::hash::Hash + Eq + Clone> DependencyGraph<K> {
    pub fn new() -> Self {
        DependencyGraph {
            parent_size_deps: HashSet::new(),
            sibling_flow_deps: HashSet::new(),
            content_deps: HashSet::new(),
            upstream_deps: HashMap::new(),
            downstream_deps: HashMap::new(),
        }
    }

    /// Mark a node as depending on parent size.
    pub fn mark_parent_size_dep(&mut self, node: K) {
        self.parent_size_deps.insert(node);
    }

    /// Mark a node as depending on sibling flow.
    pub fn mark_sibling_flow_dep(&mut self, node: K) {
        self.sibling_flow_deps.insert(node);
    }

    /// Mark a node as depending on content.
    pub fn mark_content_dep(&mut self, node: K) {
        self.content_deps.insert(node);
    }

    /// Register an upstream ancestor.
    pub fn register_upstream(&mut self, child: K, parent: K) {
        self.upstream_deps.insert(child, parent);
    }

    /// Register downstream children.
    pub fn register_downstream(&mut self, parent: K, children: Vec<K>) {
        self.downstream_deps.insert(parent, children);
    }
}

/// Represents whether an incremental layout pass was successful or requires escalation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncrementalResult {
    /// Layout converged within the specified scope.
    Converged,
    /// Layout converged but affected nodes outside the closure; requires wider pass.
    EscapedScope,
    /// Layout failed to converge; recommend full tree relayout.
    Diverged,
}

/// A cache for constraint signatures, mapping nodes to their last-known signatures.
pub struct SignatureCache<K: std::hash::Hash + Eq> {
    signatures: HashMap<K, ConstraintSignature>,
}

impl<K: std::hash::Hash + Eq> SignatureCache<K> {
    pub fn new() -> Self {
        SignatureCache {
            signatures: HashMap::new(),
        }
    }

    /// Check if a node's signature has changed.
    pub fn changed(&self, node: &K, sig: ConstraintSignature) -> bool {
        self.signatures
            .get(node)
            .map(|cached| cached != &sig)
            .unwrap_or(true) // New nodes always require layout.
    }

    /// Update the cached signature for a node.
    pub fn update(&mut self, node: K, sig: ConstraintSignature) {
        self.signatures.insert(node, sig);
    }

    /// Clear the cache.
    pub fn clear(&mut self) {
        self.signatures.clear();
    }
}

/// Incremental layout input: specifies the root and affected nodes to relayout.
#[derive(Debug, Clone)]
pub struct IncrementalInput<K> {
    /// The root node of the incremental pass.
    pub root: K,
    /// Parent layout input (must include resolved parent constraints, not just size).
    pub parent_layout_input: ParentLayoutInput,
    /// Nodes marked as dirty and requiring relayout.
    pub dirty_nodes: HashSet<K>,
    /// Optional set of nodes that must not be left modified (escalation boundary).
    pub escalation_boundary: Option<HashSet<K>>,
}

/// Parent-provided constraints for incremental root.
#[derive(Debug, Clone, Copy)]
pub struct ParentLayoutInput {
    /// Resolved parent main-axis size.
    pub parent_main: f32,
    /// Resolved parent cross-axis size.
    pub parent_cross: f32,
    /// Parent's layout type.
    pub parent_layout_type: LayoutType,
}

/// Helper function to hash floats.
fn hash_floats(values: &[f32]) -> u32 {
    let mut hash = 0u32;
    for &val in values {
        hash = hash.wrapping_mul(31).wrapping_add(val.to_bits());
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_signature() {
        // Signatures with same inputs should be equal.
        let sig1 = ConstraintSignature {
            parent_main: 100,
            parent_cross: 50,
            parent_layout_type: LayoutType::Column,
            content_size_hash: 0,
            constraint_hash: 42,
        };

        let sig2 = ConstraintSignature {
            parent_main: 100,
            parent_cross: 50,
            parent_layout_type: LayoutType::Column,
            content_size_hash: 0,
            constraint_hash: 42,
        };

        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_signature_cache() {
        let mut cache = SignatureCache::new();
        let sig = ConstraintSignature {
            parent_main: 100,
            parent_cross: 50,
            parent_layout_type: LayoutType::Column,
            content_size_hash: 0,
            constraint_hash: 42,
        };

        // First access should indicate change (new node).
        assert!(cache.changed(&"node1", sig));

        // After update, should be stable.
        cache.update("node1", sig);
        assert!(!cache.changed(&"node1", sig));

        // Different signature should indicate change.
        let sig2 = ConstraintSignature {
            parent_main: 101,
            ..sig
        };
        assert!(cache.changed(&"node1", sig2));
    }
}
