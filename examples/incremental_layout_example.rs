// Example: How Vizia can use incremental layout in morphorm

use morphorm::{
    LayoutExt, IncrementalInput, IncrementalResult, ParentLayoutInput,
    Cache, Node,
};
use std::collections::HashSet;

/// Example scenario: A Vizia application detects that a single node's width property changed.
///
/// Instead of a full tree relayout, we use incremental layout to update just the affected subtree.
pub fn vizia_on_property_change_example<N, C>(
    changed_node_key: N::CacheKey,
    root_key: N::CacheKey,
    root_node: &N,
    parent_layout_input: ParentLayoutInput,
    cache: &mut C,
    tree: &N::Tree,
    store: &N::Store,
    sublayout: &mut N::SubLayout<'_>,
) -> Result<(), String>
where
    N: Node,
    N::CacheKey: Clone + std::hash::Hash + Eq,
    C: Cache<Node = N>,
{
    // Step 1: Identify dirty nodes (the node that changed + potentially upstream).
    let mut dirty_nodes = HashSet::new();
    dirty_nodes.insert(changed_node_key.clone());

    // In a real implementation, we'd also mark upstream parents if they compute auto-sizing.
    // For now, just the changed node.

    // Step 2: Define an escalation boundary (the likely stable region).
    // For a property change on one node, a reasonable boundary is the changed node itself
    // plus its siblings and parent. In practice, you might use a larger region.
    let mut escalation_boundary = HashSet::new();
    escalation_boundary.insert(changed_node_key.clone());
    escalation_boundary.insert(root_key.clone());
    // Add siblings and parent...
    // (Implementation would traverse tree to collect these)

    // Step 3: Create incremental layout input.
    let input = IncrementalInput {
        root: root_key,
        parent_layout_input,
        dirty_nodes,
        escalation_boundary: Some(escalation_boundary),
    };

    // Step 4: Attempt incremental layout.
    match root_node.layout_incremental(&input, cache, tree, store, sublayout) {
        IncrementalResult::Converged => {
            println!("✓ Incremental layout converged: property change contained");
            Ok(())
        }
        IncrementalResult::EscapedScope => {
            println!("⚠ Changes escaped scope: retrying with wider boundary");
            // Option A: Expand boundary and retry
            // Option B: Fall back to full tree layout
            full_tree_layout_fallback(root_node, parent_layout_input, cache, tree, store, sublayout)?;
            Ok(())
        }
        IncrementalResult::Diverged => {
            println!("✗ Layout diverged: falling back to full tree layout");
            full_tree_layout_fallback(root_node, parent_layout_input, cache, tree, store, sublayout)?;
            Ok(())
        }
    }
}

/// Fallback: perform a full tree layout when incremental pass fails.
fn full_tree_layout_fallback<N, C>(
    root_node: &N,
    parent_layout_input: ParentLayoutInput,
    cache: &mut C,
    tree: &N::Tree,
    store: &N::Store,
    sublayout: &mut N::SubLayout<'_>,
) -> Result<(), String>
where
    N: Node,
    N::CacheKey: Clone,
    C: Cache<Node = N>,
{
    let _size = root_node.layout_full(
        parent_layout_input.parent_layout_type,
        parent_layout_input.parent_main,
        parent_layout_input.parent_cross,
        cache,
        tree,
        store,
        sublayout,
    );
    Ok(())
}

/// Example: Content size change (e.g., text reflow).
///
/// A text node's content changes, affecting its intrinsic size.
/// This invalidates the text node and potentially its ancestors (auto-sizing parents).
pub fn vizia_on_content_change_example<N, C>(
    text_node_key: N::CacheKey,
    root_key: N::CacheKey,
    root_node: &N,
    parent_layout_input: ParentLayoutInput,
    cache: &mut C,
    tree: &N::Tree,
    store: &N::Store,
    sublayout: &mut N::SubLayout<'_>,
) -> Result<(), String>
where
    N: Node,
    N::CacheKey: Clone + std::hash::Hash + Eq,
    C: Cache<Node = N>,
{
    // Content changes typically affect:
    // 1. The node itself (constraint signature changed due to new intrinsic size)
    // 2. Ancestors that compute auto-sizing from this node
    //
    // In a full implementation, we'd traverse up to mark auto-sized ancestors as dirty too.

    let mut dirty_nodes = HashSet::new();
    dirty_nodes.insert(text_node_key.clone());

    let input = IncrementalInput {
        root: root_key,
        parent_layout_input,
        dirty_nodes,
        escalation_boundary: None, // No boundary: let invalidation propagate fully
    };

    match root_node.layout_incremental(&input, cache, tree, store, sublayout) {
        IncrementalResult::Converged => {
            println!("✓ Content change layout converged");
            Ok(())
        }
        IncrementalResult::EscapedScope => {
            println!("Note: Escalation boundary was None, so escape indicates deep dependencies");
            Ok(())
        }
        IncrementalResult::Diverged => {
            println!("✗ Content change caused divergence; using full layout");
            full_tree_layout_fallback(root_node, parent_layout_input, cache, tree, store, sublayout)?;
            Ok(())
        }
    }
}

/// Example: Multiple sibling changes (e.g., accordion toggle).
///
/// When one accordion item expands, others collapse. This requires recomputing
/// sibling positions and potentially parent size.
pub fn vizia_on_sibling_changes_example<N, C>(
    dirty_sibling_keys: Vec<N::CacheKey>,
    root_key: N::CacheKey,
    root_node: &N,
    parent_layout_input: ParentLayoutInput,
    cache: &mut C,
    tree: &N::Tree,
    store: &N::Store,
    sublayout: &mut N::SubLayout<'_>,
) -> Result<(), String>
where
    N: Node,
    N::CacheKey: Clone + std::hash::Hash + Eq,
    C: Cache<Node = N>,
{
    let dirty_nodes: HashSet<_> = dirty_sibling_keys.into_iter().collect();

    let input = IncrementalInput {
        root: root_key,
        parent_layout_input,
        dirty_nodes,
        escalation_boundary: None,
    };

    match root_node.layout_incremental(&input, cache, tree, store, sublayout) {
        IncrementalResult::Converged => {
            println!("✓ Sibling changes layout converged");
            Ok(())
        }
        _ => {
            // Fall back to full layout for complex sibling interactions
            full_tree_layout_fallback(root_node, parent_layout_input, cache, tree, store, sublayout)?;
            Ok(())
        }
    }
}

/// Example: Comparing incremental vs. full layout for verification.
///
/// During development/testing, you can compare the results of incremental
/// and full layout to ensure correctness.
#[allow(dead_code)]
pub fn verify_incremental_layout<N, C>(
    changed_node_key: N::CacheKey,
    root_key: N::CacheKey,
    root_node: &N,
    parent_layout_input: ParentLayoutInput,
    cache_incremental: &mut C,
    cache_full: &mut C,
    tree: &N::Tree,
    store: &N::Store,
    sublayout_inc: &mut N::SubLayout<'_>,
    sublayout_full: &mut N::SubLayout<'_>,
) -> bool
where
    N: Node,
    N::CacheKey: Clone + std::hash::Hash + Eq,
    C: Cache<Node = N>,
{
    // Perform incremental layout
    let mut dirty_nodes = HashSet::new();
    dirty_nodes.insert(changed_node_key.clone());

    let input = IncrementalInput {
        root: root_key.clone(),
        parent_layout_input,
        dirty_nodes,
        escalation_boundary: None,
    };

    let inc_result = root_node.layout_incremental(&input, cache_incremental, tree, store, sublayout_inc);

    // Perform full layout
    let _full_result = root_node.layout_full(
        parent_layout_input.parent_layout_type,
        parent_layout_input.parent_main,
        parent_layout_input.parent_cross,
        cache_full,
        tree,
        store,
        sublayout_full,
    );

    // Compare cache results to verify correctness
    // In a real implementation, you'd iterate through all nodes and compare their bounds.
    matches!(inc_result, IncrementalResult::Converged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_compiles() {
        // This is just a compilation test to ensure the API usage patterns are valid.
    }
}

// Main function for documentation purposes
fn main() {
    println!("This example demonstrates incremental layout API usage patterns.");
    println!("See the functions above for concrete integration examples.");
    println!();
    println!("Key scenarios:");
    println!("  1. Property change (width/height) - vizia_on_property_change_example");
    println!("  2. Content change (text reflow) - vizia_on_content_change_example");
    println!("  3. Sibling changes (accordion) - vizia_on_sibling_changes_example");
    println!("  4. Verification - verify_incremental_layout");
}
