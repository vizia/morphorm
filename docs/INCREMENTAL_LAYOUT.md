# Incremental Layout in Morphorm

## Overview

Morphorm now supports **incremental layout**, enabling stable, context-aware subtree relayout without full-tree recomputation. This addresses the core issue that subtree-only relayout was losing ancestor constraint state and sibling flow information, causing inconsistent bounds and disappearing views.

## Problem Statement

The traditional full-tree layout algorithm in Morphorm works reliably but is expensive for large trees when only a small portion changes. Previous attempts at "incremental" or "subtree" layout failed because they:

1. **Lost ancestor constraints**: Subtree roots didn't receive the resolved parent constraint values from their ancestors.
2. **Skipped sibling recomputation**: When a sibling's size changed, dependent siblings weren't invalidated.
3. **Had no dependency model**: Changes propagated ad hoc, making it impossible to determine if a local pass was truly complete.
4. **Lacked cache signatures**: Without knowing which inputs produced a layout, caches couldn't distinguish stable from stale nodes.

## Key Concepts

### 1. Constraint Signatures

Each node caches a **constraint signature** — a compact record of the resolved inputs that produced its layout:

```rust
pub struct ConstraintSignature {
    pub parent_main: u32,           // Resolved parent main-axis size
    pub parent_cross: u32,          // Resolved parent cross-axis size
    pub parent_layout_type: LayoutType,
    pub content_size_hash: u32,     // Hash of intrinsic content size (if present)
    pub constraint_hash: u32,       // Hash of min/max constraints
}
```

A node is **skipped** if its signature hasn't changed. This prevents redundant layout while still catching constraint propagation.

### 2. Dependency Tracking

The **dependency graph** tracks which nodes depend on what:

```rust
pub struct DependencyGraph<K> {
    pub parent_size_deps: HashSet<K>,    // Nodes using % or Stretch(n)
    pub sibling_flow_deps: HashSet<K>,   // Nodes affected by sibling order/size
    pub content_deps: HashSet<K>,        // Nodes with intrinsic content
    pub upstream_deps: HashMap<K, K>,    // child -> parent
    pub downstream_deps: HashMap<K, Vec<K>>, // parent -> children
}
```

When a node changes:
- **Downstream nodes** (children, siblings) are invalidated if they depend on parent size or sibling flow.
- **Upstream nodes** (parents) are invalidated if they compute auto-sizing from children.

### 3. Incremental Entry Point

```rust
pub struct IncrementalInput<K> {
    pub root: K,
    pub parent_layout_input: ParentLayoutInput,  // Resolved ancestor constraints
    pub dirty_nodes: HashSet<K>,                 // Nodes marked for relayout
    pub escalation_boundary: Option<HashSet<K>>, // Max scope for this pass
}

pub struct ParentLayoutInput {
    pub parent_main: f32,
    pub parent_cross: f32,
    pub parent_layout_type: LayoutType,
}
```

The **parent_layout_input** is crucial: it provides the fully resolved parent constraints that subtree roots need. This is not just the parent's size, but the constraints the parent applies to the subtree.

### 4. Escalation Contract

```rust
pub enum IncrementalResult {
    Converged,      // Layout stable within scope
    EscapedScope,   // Changes propagated beyond boundary → need wider pass
    Diverged,       // Failed to converge → recommend full relayout
}
```

If changes escape the escalation boundary, the caller knows to retry with a larger scope (up the tree). This prevents silent inconsistency.

### 5. Fixed-Point Iteration

The incremental engine uses **fixed-point iteration**:

```
while affected_nodes not empty and iteration < max_iterations:
    for each affected_node:
        relayout(affected_node)
    affected_nodes = compute_invalidation_from_changes()
```

This ensures that cascading changes (e.g., parent expands → children stretch → siblings reposition) eventually settle.

## Usage

### Basic Incremental Layout

```rust
use morphorm::{LayoutExt, IncrementalInput, ParentLayoutInput, LayoutType};
use std::collections::HashSet;

// Mark specific nodes as dirty
let mut dirty_nodes = HashSet::new();
dirty_nodes.insert(node_key);

// Provide resolved parent constraints
let input = IncrementalInput {
    root: subtree_root_key,
    parent_layout_input: ParentLayoutInput {
        parent_main: 800.0,
        parent_cross: 600.0,
        parent_layout_type: LayoutType::Column,
    },
    dirty_nodes,
    escalation_boundary: Some(subtree_keys), // Optional: limit scope
};

// Perform the incremental pass
match root_node.layout_incremental(&input, &mut cache, &tree, &store, &mut sublayout) {
    IncrementalResult::Converged => {
        // Success: layout is stable within scope
    }
    IncrementalResult::EscapedScope => {
        // Changes affected nodes outside boundary; retry with wider scope
        // Option: Include parent node and retry, or fall back to full layout
    }
    IncrementalResult::Diverged => {
        // Failed to converge; perform full tree layout
        root_node.layout_full(parent_layout_type, parent_main, parent_cross, &mut cache, &tree, &store, &mut sublayout);
    }
}
```

### Subtree Layout with Explicit Constraints

```rust
// When you know parent constraints but not the tree structure
let parent_input = ParentLayoutInput {
    parent_main: 400.0,
    parent_cross: 300.0,
    parent_layout_type: LayoutType::Row,
};

let size = subtree_node.layout_subtree(
    parent_input,
    &mut cache,
    &tree,
    &store,
    &mut sublayout,
);
```

### Full-Tree Layout (Explicit)

```rust
// Traditional full relayout; still available for comparison or forced refresh
let size = root_node.layout_full(
    LayoutType::Column,
    800.0,
    600.0,
    &mut cache,
    &tree,
    &store,
    &mut sublayout,
);
```

## Implementation Details

### In Vizia

The Vizia UI framework can now use incremental layout more effectively:

1. **On property change** (e.g., `width` property updated):
   - Mark the node and its ancestors as dirty.
   - Determine if changes affect only a local subtree (check dependencies).
   - Call `layout_incremental()` with an escalation boundary at the likely stable ancestor.

2. **On layout escape**:
   - Expand the boundary to include the escaping nodes.
   - Retry `layout_incremental()` with the wider boundary.
   - If still escaping, escalate to full tree layout.

3. **Before rendering**:
   - Check `IncrementalResult` to verify layout stability.
   - If converged, render the updated subtree.
   - If escaped or diverged, retry with appropriate scope.

### Cache Signature Integration

The layout engine tracks signatures at key decision points:

- After computing a node's size (before laying out children).
- After resolving stretch factors and auto-sizing.
- Before and after overlay/grid/wrap passes.

Nodes with unchanged signatures skip descendant traversal, dramatically reducing work for stable regions.

## Trade-offs and Tuning

### When to Use Each Method

| Method | Use When | Cost | Safety |
|--------|----------|------|--------|
| `layout_incremental()` | Single node changed | Minimal | High (with escalation) |
| `layout_subtree()` | Know parent constraints | Low | High |
| `layout_full()` | Multiple changes, uncertain scope | High | Highest |

### Escalation Boundary

- **Too narrow**: Many `EscapedScope` results → overhead from retries.
- **Too wide**: Defeats the purpose of incremental layout.
- **Good heuristic**: Include the common ancestor of all dirty nodes plus one level up.

### Max Iterations

Default is 10 iterations (tunable via `IncrementalLayoutEngine::set_max_iterations()`).

- 10 iterations handles most practical layouts (auto-sizing chains up to ~10 levels).
- If `Diverged` is common, check for cycles in constraint dependencies.

## Future Enhancements

1. **Stateful signature cache**: Track signatures across frames to minimize recomputation.
2. **Lazy dependency graph**: Build dependencies on-demand during layout.
3. **Parallel subtree passes**: If dependencies permit, parallelize sibling subtree layout.
4. **Adaptive escalation**: Automatically widen boundaries based on `EscapedScope` patterns.
5. **Debug visualization**: Tool to show which nodes are being relayout and why.

## Testing

Incremental layout is tested against the full layout algorithm across:

- Property changes (width, height, constraints).
- Sibling reordering.
- Content size updates.
- Percentage and stretch unit changes.
- Nested layout types (Row/Column/Grid/Overlay).
- RTL and text wrapping.

All tests verify that `layout_incremental(scope) == layout_full(tree)` for identical input changes.
