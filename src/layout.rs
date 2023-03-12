// Temp
#![allow(dead_code)]

use smallvec::SmallVec;

use crate::{Cache, LayoutType, Node, NodeExt, PositionType, Size, Units};
use crate::{CacheExt, Units::*};

const DEFAULT_MIN: f32 = -f32::MAX;
const DEFAULT_MAX: f32 = f32::MAX;

#[derive(Debug, Clone, Copy)]
enum Axis {
    MainBefore,
    Main,
    MainAfter,
}

#[derive(Copy, Clone)]
struct StretchNode<'a, N: Node> {
    // A reference to the node.
    node: &'a N,
    // The index of the node.
    index: usize,
    // The stretch factor of the node.
    factor: f32,

    axis: Axis,

    violation: f32,

    frozen: bool,
}

#[derive(Debug, Copy, Clone)]
struct ChildNode<'a, N: Node> {
    // A reference to the node.
    node: &'a N,
    // Sum of the flex factors on the main axis of the node.
    main_flex_sum: f32,
    // Sum of non-stretch space on the main axis of the node.
    main_non_flex: f32,
    // A remainder used during stretch computation.
    main_remainder: f32,
    // Sum of the cross_before, cross, and cross_after flex factors of the node.
    cross_flex_sum: f32,
    // Sum of non-stretch space on the cross axis of the node.
    cross_non_flex: f32,
    // Computed cross space of the node.
    cross: f32,
    // Computed main-before space of the node.
    main_before: f32,
    // Computed main-after space of the node.
    main_after: f32,
    // Computed cross-before space of the node.
    cross_before: f32,
    // Computed cross-after space of the node.
    cross_after: f32,
}

/// Performs layout on the given node returning its computed size.
///
/// The algorithm recurses down the tree, in depth-first order, and performs
/// layout on every node starting from the input `node`.
///
/// # Arguments
///
/// * `node` - Root node to start layout from.
/// * `parent_layout_type` - The [`LayoutType`] of the parent of the `node`.
/// * `parent_main` - The size of the parent of the `node` on its main axis. If the `node` has no parent then pass the size of the node.
/// * `cross_size` - The size of the `node` along its cross axis.
/// * `cache` - A mutable reference to the [`Cache`].
/// * `tree` - A mutable reference to the [`Tree`](crate::Node::Tree).
/// * `store` - A mutable reference to the [`Store`](crate::Node::Store).
///
/// # Example
///
/// ```
/// layout(&root, LayoutType::Column, 600.0, 600.0, &mut cache, &tree, &store);
/// ```
pub(crate) fn layout<N, C>(
    node: &N,
    parent_layout_type: LayoutType,
    parent_main: f32,
    cross_size: f32,
    cache: &mut C,
    tree: &<N as Node>::Tree,
    store: &<N as Node>::Store,
) -> Size
where
    N: Node,
    C: Cache<Node = N>,
{
    // The layout type of the node. Determines the main and cross axes of the children.
    let layout_type = node.layout_type(store).unwrap_or_default();

    // The desired main-axis and cross-axis sizes of the node.
    let main = node.main(store, parent_layout_type);
    let cross = node.cross(store, parent_layout_type);

    // Compute main-axis size.
    let mut computed_main = match main {
        Pixels(val) => val,
        Percentage(val) => (parent_main * (val / 100.0)).round(),
        Stretch(_) => parent_main,
        Auto => 0.0,
    };

    // Cross size is determined by the parent.
    let mut computed_cross = cross_size;

    // Get the total number of children of the node.
    let num_children = node.children(tree).count();

    // Apply content sizing.
    if (main == Auto || cross == Auto) && num_children == 0 {
        let parent_main = if main == Auto { None } else { Some(computed_main) };
        let parent_cross = if cross == Auto { None } else { Some(computed_cross) };
        if let Some(content_size) = node.content_sizing(store, parent_layout_type, parent_main, parent_cross) {
            computed_main = content_size.0;
            computed_cross = content_size.1;
        }
    }

    // Apply main-axis size constraints for pixels and percentage.
    let min_main = node.min_main(store, parent_layout_type).to_px(parent_main, DEFAULT_MIN);
    let max_main = node.max_main(store, parent_layout_type).to_px(parent_main, DEFAULT_MAX);
    computed_main = computed_main.clamp(min_main, max_main);

    // TODO: Figure out how to constrain content size on cross axis.

    // Determine the parent_main/cross size to pass to the children based on the layout type of the parent and the node.
    // i.e. if the parent layout type and the node layout type are different, swap the main and the cross axes.
    let (mut parent_main, mut parent_cross) = if parent_layout_type == layout_type {
        (computed_main, computed_cross)
    } else {
        (computed_cross, computed_main)
    };

    // Sum of all space and size flex factors on the main-axis of the node.
    let mut main_flex_sum = 0.0;

    // Sum of all child nodes on the main-axis.
    let mut main_sum = 0.0;

    // Maximum of all child nodes on the cross-axis.
    let mut cross_max = 0.0f32;

    // List of child nodes for the current node.
    let mut children = SmallVec::<[ChildNode<N>; 3]>::new();

    // List of stretch nodes for the current node.
    // A stretch node is any flexible space/size. e.g. main_before, main, and main_after are separate stretch nodes
    let mut stretch_nodes = SmallVec::<[StretchNode<N>; 3]>::new();

    // Parent overrides for child auto space.
    let node_child_main_before = node.child_main_before(store, layout_type);
    let node_child_main_after = node.child_main_after(store, layout_type);
    let node_child_cross_before = node.child_cross_before(store, layout_type);
    let node_child_cross_after = node.child_cross_after(store, layout_type);
    let node_child_main_between = node.main_between(store, layout_type);

    // Determine index of first and last parent-directed child nodes.
    let mut iter = node
        .children(tree)
        .enumerate()
        .filter(|(_, child)| child.position_type(store).unwrap_or_default() == PositionType::ParentDirected);

    let first = iter.next().map(|(index, _)| index);
    let last = iter.last().map_or(first, |(index, _)| Some(index));

    // Compute space and size of non-flexible children.
    for (index, child) in node.children(tree).enumerate() {
        let child_position_type = child.position_type(store).unwrap_or_default();

        // Get desired space and size.
        let mut child_main_before = child.main_before(store, layout_type);
        let child_main = child.main(store, layout_type);
        let mut child_main_after = child.main_after(store, layout_type);

        let mut child_cross_before = child.cross_before(store, layout_type);
        let child_cross = child.cross(store, layout_type);
        let mut child_cross_after = child.cross_after(store, layout_type);

        // Get fixed-size space and size constraints.
        let child_min_cross_before = child.min_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross_before = child.max_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        let child_min_cross = child.min_cross(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross = child.max_cross(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        let child_min_cross_after = child.min_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross_after = child.max_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        let child_min_main_before = child.min_main_before(store, layout_type).to_px(parent_main, DEFAULT_MIN);
        let child_max_main_before = child.max_main_before(store, layout_type).to_px(parent_main, DEFAULT_MAX);

        let child_min_main_after = child.min_main_after(store, layout_type).to_px(parent_main, DEFAULT_MIN);
        let child_max_main_after = child.max_main_after(store, layout_type).to_px(parent_main, DEFAULT_MAX);

        // Apply parent child_space overrides to auto child space.
        if child_main_before == Units::Auto {
            if first == Some(index) || child_position_type == PositionType::SelfDirected {
                child_main_before = node_child_main_before;
            } else {
                child_main_before = node_child_main_between;
            }
        }

        if child_main_after == Units::Auto && (last == Some(index) || child_position_type == PositionType::SelfDirected)
        {
            child_main_after = node_child_main_after;
        }

        if child_cross_before == Units::Auto {
            child_cross_before = node_child_cross_before;
        }

        if child_cross_after == Units::Auto {
            child_cross_after = node_child_cross_after;
        }

        // Sum flex factors on the cross axis of the child node.
        let mut child_cross_flex_sum = 0.0;

        if let Stretch(factor) = child_cross_before {
            child_cross_flex_sum += factor;
        }

        if let Stretch(factor) = child_cross {
            child_cross_flex_sum += factor;
        }

        if let Stretch(factor) = child_cross_after {
            child_cross_flex_sum += factor;
        }

        // Sum flex factors on the main axis of the child node.
        let mut child_main_flex_sum = 0.0;

        if let Stretch(factor) = child_main_before {
            child_main_flex_sum += factor;

            stretch_nodes.push(StretchNode {
                node: child,
                index,
                factor,
                axis: Axis::MainBefore,
                violation: 0.0,
                frozen: false,
            });
        }

        if let Stretch(factor) = child_main {
            child_main_flex_sum += factor;

            stretch_nodes.push(StretchNode {
                node: child,
                index,
                factor,
                axis: Axis::Main,
                violation: 0.0,
                frozen: false,
            });
        }

        if let Stretch(factor) = child_main_after {
            child_main_flex_sum += factor;

            stretch_nodes.push(StretchNode {
                node: child,
                index,
                factor,
                axis: Axis::MainAfter,
                violation: 0.0,
                frozen: false,
            });
        }

        // Compute fixed-size child cross_before.
        let computed_child_cross_before = if !child_cross_before.is_stretch() {
            child_cross_before.to_px(parent_cross, 0.0).clamp(child_min_cross_before, child_max_cross_before)
        } else {
            0.0
        };

        // Compute fixed-size child_cross.
        let mut computed_child_cross = 0.0f32;

        if !child_cross.is_stretch() {
            computed_child_cross = child_cross.to_px(parent_cross, 0.0).clamp(child_min_cross, child_max_cross);
        }

        // Compute fixed-size child cross_after.
        let computed_child_cross_after = if !child_cross_after.is_stretch() {
            child_cross_after.to_px(parent_cross, 0.0).clamp(child_min_cross_after, child_max_cross_after)
        } else {
            0.0
        };

        // Compute fixed-size child main_before.
        let computed_child_main_before =
            child_main_before.to_px(parent_main, 0.0).clamp(child_min_main_before, child_max_main_before);

        // Compute fixed-size child main_after.
        let computed_child_main_after =
            child_main_after.to_px(parent_main, 0.0).clamp(child_min_main_after, child_max_main_after);

        let mut computed_child_main = 0.0;

        // Compute fixed-size child main.
        if !child_main.is_stretch() && !child_cross.is_stretch() {
            let child_size = layout(child, layout_type, parent_main, computed_child_cross, cache, tree, store);

            computed_child_main = child_size.main;
            computed_child_cross = child_size.cross;
        }

        // Total computed size on the cross-axis of the child.
        let child_cross_non_flex = computed_child_cross_before + computed_child_cross + computed_child_cross_after;

        // Total computed size on the main-axis of the child.
        let child_main_non_flex = computed_child_main_before + computed_child_main + computed_child_main_after;

        if child_position_type == PositionType::ParentDirected {
            main_flex_sum += child_main_flex_sum;
            main_sum += child_main_non_flex;
            cross_max = cross_max.max(child_cross_non_flex);
        }

        children.push(ChildNode {
            node: child,
            main_flex_sum: child_main_flex_sum,
            main_non_flex: child_main_non_flex,
            main_remainder: 0.0,
            cross_flex_sum: child_cross_flex_sum,
            cross_non_flex: child_cross_non_flex,
            cross: computed_child_cross,
            main_before: computed_child_main_before,
            main_after: computed_child_main_after,
            cross_before: computed_child_cross_before,
            cross_after: computed_child_cross_after,
        });
    }

    // Determine cross-size of auto node from children.
    if num_children != 0 && node.cross(store, layout_type) == Auto {
        parent_cross = cross_max;
    }

    // Compute flexible space and size on the cross-axis.
    for (index, child) in children.iter_mut().enumerate() {
        let mut child_cross_before = child.node.cross_before(store, layout_type);
        let child_cross = child.node.cross(store, layout_type);
        let mut child_cross_after = child.node.cross_after(store, layout_type);

        // Apply child_space overrides.
        if child_cross_before == Units::Auto {
            child_cross_before = node_child_cross_before;
        }

        if child_cross_after == Units::Auto {
            child_cross_after = node_child_cross_after;
        }

        // Collect stretch cross items.
        let mut cross_axis = SmallVec::<[StretchNode<N>; 3]>::new();
        if let Stretch(factor) = child_cross_before {
            cross_axis.push(StretchNode {
                node: child.node,
                index,
                factor,
                axis: Axis::MainBefore,
                violation: 0.0,
                frozen: false,
            });
        }
        if let Stretch(factor) = child_cross {
            cross_axis.push(StretchNode {
                node: child.node,
                index,
                factor,
                axis: Axis::Main,
                violation: 0.0,
                frozen: false,
            });
        }
        if let Stretch(factor) = child_cross_after {
            cross_axis.push(StretchNode {
                node: child.node,
                index,
                factor,
                axis: Axis::MainAfter,
                violation: 0.0,
                frozen: false,
            });
        }

        loop {
            // If all stretch items are frozen, exit the loop.
            if cross_axis.iter().all(|item| item.frozen) {
                break;
            }

            // Compute free space in the cross axis.
            let child_cross_free_space = parent_cross - child.cross_non_flex;

            // Total size violation in the cross axis.
            let mut total_violation = 0.0;

            for item in cross_axis.iter_mut().filter(|item| !item.frozen) {
                match item.axis {
                    // TODO - Refactor to reduce this code duplication.
                    Axis::MainBefore => {
                        let actual_cross = (item.factor * child_cross_free_space / child.cross_flex_sum).round();

                        let child_min_cross_before =
                            item.node.min_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
                        let child_max_cross_before =
                            item.node.max_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

                        let clamped = actual_cross.clamp(child_min_cross_before, child_max_cross_before);
                        item.violation = clamped - actual_cross;
                        total_violation += clamped - actual_cross;

                        child.cross_before = clamped;
                    }

                    Axis::Main => {
                        let actual_cross = (item.factor * child_cross_free_space / child.cross_flex_sum).round();

                        let child_min_cross = item.node.min_cross(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
                        let child_max_cross = item.node.max_cross(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

                        let clamped = actual_cross.clamp(child_min_cross, child_max_cross);
                        item.violation = clamped - actual_cross;
                        total_violation += clamped - actual_cross;

                        child.cross = clamped;
                    }

                    Axis::MainAfter => {
                        let actual_cross = (item.factor * child_cross_free_space / child.cross_flex_sum).round();

                        let child_min_cross_after =
                            item.node.min_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
                        let child_max_cross_after =
                            item.node.max_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

                        let clamped = actual_cross.clamp(child_min_cross_after, child_max_cross_after);
                        item.violation = clamped - actual_cross;
                        total_violation += clamped - actual_cross;

                        child.cross_after = clamped;
                    }
                }
            }

            for item in cross_axis.iter_mut().filter(|item| !item.frozen) {
                // Freeze over-stretched items.
                item.frozen = match total_violation {
                    v if v > 0.0 => item.violation > 0.0,
                    v if v < 0.0 => item.violation < 0.0,
                    _ => true,
                };

                // If the item is frozen, adjust the used_space and sum of cross stretch factors.
                if item.frozen {
                    match item.axis {
                        Axis::MainBefore => child.cross_non_flex += child.cross_before,
                        Axis::Main => child.cross_non_flex += child.cross,
                        Axis::MainAfter => child.cross_non_flex += child.cross_after,
                    }
                    child.cross_flex_sum -= item.factor;
                }
            }

            let child_position_type = child.node.position_type(store).unwrap_or_default();
            if child_position_type == PositionType::ParentDirected {
                cross_max = cross_max.max(child.cross_non_flex);
            }

            // Determine cross-size of auto node from children.
            if num_children != 0 && node.cross(store, layout_type) == Auto {
                parent_cross = cross_max;
            }
        }

        if let Stretch(_) = child_cross {
            if !child.node.main(store, layout_type).is_stretch() {
                let size = layout(child.node, layout_type, computed_main, child.cross, cache, tree, store);

                child.main_non_flex += size.main;

                let child_position_type = child.node.position_type(store).unwrap_or_default();
                if child_position_type == PositionType::ParentDirected {
                    cross_max = cross_max.max(size.cross);
                    main_sum += size.main;
                }
            }
        }
    }

    // Determine main-size of auto node from children.
    if num_children != 0 && node.main(store, layout_type) == Auto {
        parent_main = parent_main.max(main_sum);
    }

    // Calculate free space on the main-axis.
    let free_main_space = parent_main - main_sum;
    let main_px_per_flex = free_main_space / main_flex_sum;
    let mut remainder: f32 = 0.0;

    // Compute flexible space and size on the main axis.
    for item in stretch_nodes.iter() {
        let child_position_type = item.node.position_type(store).unwrap_or_default();

        let actual_main = if child_position_type == PositionType::SelfDirected {
            let child_main_free_space = parent_main - children[item.index].main_non_flex;
            let px_per_flex = child_main_free_space / children[item.index].main_flex_sum;
            let desired_main = item.factor * px_per_flex + children[item.index].main_remainder;
            let actual_main = desired_main.round();
            children[item.index].main_remainder = desired_main - actual_main;
            actual_main
        } else {
            let desired_main = item.factor * main_px_per_flex + remainder;
            let actual_main = desired_main.round();
            remainder = desired_main - actual_main;
            actual_main
        };

        match item.axis {
            Axis::MainBefore => {
                children[item.index].main_before = actual_main;
                if child_position_type == PositionType::ParentDirected {
                    main_sum += actual_main;
                }
            }

            Axis::MainAfter => {
                children[item.index].main_after = actual_main;
                if child_position_type == PositionType::ParentDirected {
                    main_sum += actual_main;
                }
            }

            Axis::Main => {
                let computed_child_cross = children[item.index].cross;
                let size = layout(item.node, layout_type, actual_main, computed_child_cross, cache, tree, store);
                if child_position_type == PositionType::ParentDirected {
                    cross_max = cross_max.max(size.cross);
                    main_sum += size.main;
                }
            }
        }
    }

    // Position children.
    let mut main_pos = 0.0;
    for child in children.iter() {
        let child_position_type = child.node.position_type(store).unwrap_or_default();

        match child_position_type {
            PositionType::SelfDirected => {
                cache.set_pos(child.node, layout_type, child.main_before, child.cross_before);
            }

            PositionType::ParentDirected => {
                main_pos += child.main_before;
                cache.set_pos(child.node, layout_type, main_pos, child.cross_before);
                main_pos += cache.main(child.node, layout_type) + child.main_after;
            }
        };
    }

    // Determine size of auto node from children
    if num_children != 0 {
        let (main_sum, cross_max) =
            if parent_layout_type == layout_type { (main_sum, cross_max) } else { (cross_max, main_sum) };

        if main == Auto {
            computed_main = main_sum;
        }

        if cross == Auto {
            computed_cross = cross_max;
        }
    }

    // Set the computed size of the node in the cache.
    cache.set_size(node, parent_layout_type, computed_main, computed_cross);

    // Return the computed size, propagating it back up the tree.
    Size { main: computed_main, cross: computed_cross }
}
