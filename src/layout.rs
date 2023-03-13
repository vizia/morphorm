use smallvec::SmallVec;

use crate::{Cache, LayoutType, Node, NodeExt, PositionType, Size, Units};
use crate::{CacheExt, Units::*};

const DEFAULT_MIN: f32 = -f32::MAX;
const DEFAULT_MAX: f32 = f32::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemType {
    Before,
    Size,
    After,
}

#[derive(Copy, Clone)]
struct StretchItem {
    // The index of the node.
    index: usize,
    // The stretch factor of the node.
    factor: f32,
    // The type of stretch item, either space-before, size, or space-after.
    item_type: ItemType,
    // The violation of the stretch item after clamping.
    violation: f32,

    frozen: bool,

    min: f32,
    max: f32,
}

impl StretchItem {
    pub fn new(index: usize, factor: f32, item_type: ItemType, min: f32, max: f32) -> Self {
        Self { index, factor, item_type, violation: 0.0, frozen: false, min, max }
    }
}

#[derive(Debug, Copy, Clone)]
struct ChildNode<'a, N: Node> {
    // A reference to the node.
    node: &'a N,
    // Sum of the flex factors on the main axis of the node.
    main_flex_sum: f32,
    // Sum of non-stretch space on the main axis of the node.
    main_non_flex: f32,
    // Sum of the cross_before, cross, and cross_after flex factors of the node.
    cross_flex_sum: f32,
    // Sum of non-stretch space on the cross axis of the node.
    cross_non_flex: f32,
    // Computed cross size of the node.
    cross: f32,
    // Computed main-before space of the node.
    main_before: f32,
    // Computed main-after space of the node.
    main_after: f32,
    // Computed main size of the node.
    main: f32,
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
    let mut children = SmallVec::<[ChildNode<N>; 32]>::with_capacity(num_children);


    // List of stretch nodes for the current node.
    // A stretch node is any flexible space/size. e.g. main_before, main, and main_after are separate stretch nodes
    let mut stretch_nodes = SmallVec::<[StretchItem; 32]>::new();

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
        let child_min_cross_before = child.min_cross_before(store, layout_type);
        let child_max_cross_before = child.max_cross_before(store, layout_type);

        let child_min_cross = child.min_cross(store, layout_type);
        let child_max_cross = child.max_cross(store, layout_type);

        let child_min_cross_after = child.min_cross_after(store, layout_type);
        let child_max_cross_after = child.max_cross_after(store, layout_type);

        let child_min_main_before = child.min_main_before(store, layout_type);
        let child_max_main_before = child.max_main_before(store, layout_type);

        let child_min_main_after = child.min_main_after(store, layout_type);
        let child_max_main_after = child.max_main_after(store, layout_type);

        let child_min_main = child.min_main(store, layout_type);
        let child_max_main = child.max_main(store, layout_type);

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

            if child_position_type == PositionType::ParentDirected {
                stretch_nodes.push(StretchItem::new(
                    index,
                    factor,
                    ItemType::Before,
                    child_min_main_before.to_px(parent_main, DEFAULT_MIN),
                    child_max_main_before.to_px(parent_main, DEFAULT_MAX),
                ));
            }
        }

        if let Stretch(factor) = child_main {
            child_main_flex_sum += factor;

            if child_position_type == PositionType::ParentDirected {
                stretch_nodes.push(StretchItem::new(
                    index,
                    factor,
                    ItemType::Size,
                    child_min_main.to_px(parent_main, DEFAULT_MIN),
                    child_max_main.to_px(parent_main, DEFAULT_MAX),
                ));
            }
        }

        if let Stretch(factor) = child_main_after {
            child_main_flex_sum += factor;

            if child_position_type == PositionType::ParentDirected {
                stretch_nodes.push(StretchItem::new(
                    index,
                    factor,
                    ItemType::After,
                    child_min_main_after.to_px(parent_main, DEFAULT_MIN),
                    child_max_main_after.to_px(parent_main, DEFAULT_MAX),
                ));
            }
        }

        // Compute fixed-size child cross_before.
        let computed_child_cross_before =
            child_cross_before.to_px_clamped(parent_cross, 0.0, child_min_cross_before, child_max_cross_before);

        // Compute fixed-size child_cross.
        let mut computed_child_cross = child_cross.to_px_clamped(parent_cross, 0.0, child_min_cross, child_max_cross);

        // Compute fixed-size child cross_after.
        let computed_child_cross_after =
            child_cross_after.to_px_clamped(parent_cross, 0.0, child_min_cross_after, child_max_cross_after);

        // Compute fixed-size child main_before.
        let computed_child_main_before =
            child_main_before.to_px_clamped(parent_main, 0.0, child_min_main_before, child_max_main_before);

        // Compute fixed-size child main_after.
        let computed_child_main_after =
            child_main_after.to_px_clamped(parent_main, 0.0, child_min_main_after, child_max_main_after);

        let mut computed_child_main = child_main.to_px_clamped(parent_main, 0.0, child_min_main, child_max_main);

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
            cross_flex_sum: child_cross_flex_sum,
            cross_non_flex: child_cross_non_flex,
            cross_before: computed_child_cross_before,
            cross: computed_child_cross,
            cross_after: computed_child_cross_after,
            main_before: computed_child_main_before,
            main: computed_child_main,
            main_after: computed_child_main_after,
        });
    }

    // Determine cross-size of auto node from children.
    if num_children != 0 && node.cross(store, layout_type) == Auto {
        parent_cross = cross_max;
    }

    // Compute flexible space and size on the cross-axis for both parent-directed and self-directed nodes.
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
        let mut cross_axis = SmallVec::<[StretchItem; 3]>::new();
        if let Stretch(factor) = child_cross_before {
            let child_min_cross_before =
                child.node.min_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
            let child_max_cross_before =
                child.node.max_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

            cross_axis.push(StretchItem::new(
                index,
                factor,
                ItemType::Before,
                child_min_cross_before,
                child_max_cross_before,
            ));
        }
        if let Stretch(factor) = child_cross {
            let child_min_cross = child.node.min_cross(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
            let child_max_cross = child.node.max_cross(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

            cross_axis.push(StretchItem::new(index, factor, ItemType::Size, child_min_cross, child_max_cross));
        }
        if let Stretch(factor) = child_cross_after {
            let child_min_cross_after = child.node.min_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
            let child_max_cross_after = child.node.max_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

            cross_axis.push(StretchItem::new(
                index,
                factor,
                ItemType::After,
                child_min_cross_after,
                child_max_cross_after,
            ));
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
                let actual_cross = (item.factor * child_cross_free_space / child.cross_flex_sum).round();

                let clamped = actual_cross.clamp(item.min, item.max);
                item.violation = clamped - actual_cross;
                total_violation += item.violation;

                match item.item_type {
                    ItemType::Before => child.cross_before = clamped,
                    ItemType::Size => child.cross = clamped,
                    ItemType::After => child.cross_after = clamped,
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
                    child.cross_flex_sum -= item.factor;
                }
            }
        }

        if child_cross.is_stretch() && !child.node.main(store, layout_type).is_stretch() {
            let child_size = layout(child.node, layout_type, computed_main, child.cross, cache, tree, store);

            child.main_non_flex += child_size.main;
            child.main = child_size.main;

            let child_position_type = child.node.position_type(store).unwrap_or_default();
            if child_position_type == PositionType::ParentDirected {
                cross_max = cross_max.max(child_size.cross);
                main_sum += child_size.main;
            }
        }
    }

    // Determine main-size of auto node from children.
    if num_children != 0 && node.main(store, layout_type) == Auto {
        parent_main = parent_main.max(main_sum);
    }

    // Compute flexible space and size on the main axis.
    if !stretch_nodes.is_empty() {
        loop {
            if stretch_nodes.iter().all(|item| item.frozen) {
                break;
            }

            // Calculate free space on the main-axis.
            let free_main_space = parent_main - main_sum;

            let mut total_violation = 0.0;

            for item in stretch_nodes.iter_mut().filter(|item| !item.frozen) {
                let child = &mut children[item.index];

                let actual_main = (item.factor * free_main_space / main_flex_sum).round();

                let clamped = actual_main.clamp(item.min, item.max);
                item.violation = clamped - actual_main;
                total_violation += item.violation;
                match item.item_type {
                    ItemType::Before => child.main_before = clamped,
                    ItemType::Size => child.main = clamped,
                    ItemType::After => child.main_after = clamped,
                }
            }

            for item in stretch_nodes.iter_mut().filter(|item| !item.frozen) {
                let child = &mut children[item.index];

                // Freeze over-stretched items.
                item.frozen = match total_violation {
                    total if total > 0.0 => item.violation > 0.0,
                    total if total < 0.0 => item.violation < 0.0,
                    _ => true,
                };

                // If the item is frozen, adjust the used_space and sum of cross stretch factors.
                if item.frozen {
                    main_flex_sum -= item.factor;

                    if item.item_type == ItemType::Size {
                        let child_size = layout(child.node, layout_type, child.main, child.cross, cache, tree, store);
                        cross_max = cross_max.max(child_size.cross);
                        main_sum += child_size.main - child.main;
                        child.main = child_size.main;
                    }
                }
            }
        }
    }

    // Compute flexible space and size on the main-axis for self-directed nodes.
    for (index, child) in children
        .iter_mut()
        .filter(|child| child.node.position_type(store).unwrap_or_default() == PositionType::SelfDirected)
        .enumerate()
    {
        let mut child_main_before = child.node.main_before(store, layout_type);
        let child_main = child.node.main(store, layout_type);
        let mut child_main_after = child.node.main_after(store, layout_type);

        // Apply child_space overrides.
        if child_main_before == Units::Auto {
            child_main_before = node_child_main_before;
        }

        if child_main_after == Units::Auto {
            child_main_after = node_child_main_after;
        }

        // Collect stretch main items.
        let mut main_axis = SmallVec::<[StretchItem; 3]>::new();
        if let Stretch(factor) = child_main_before {
            let child_min_main_before = child.node.min_main_before(store, layout_type).to_px(parent_main, DEFAULT_MIN);
            let child_max_main_before = child.node.max_main_before(store, layout_type).to_px(parent_main, DEFAULT_MAX);

            main_axis.push(StretchItem::new(
                index,
                factor,
                ItemType::Before,
                child_min_main_before,
                child_max_main_before,
            ));
        }
        if let Stretch(factor) = child_main {
            let child_min_main = child.node.min_main(store, layout_type).to_px(parent_main, DEFAULT_MIN);
            let child_max_main = child.node.max_main(store, layout_type).to_px(parent_main, DEFAULT_MAX);

            main_axis.push(StretchItem::new(index, factor, ItemType::Size, child_min_main, child_max_main));
        }
        if let Stretch(factor) = child_main_after {
            let child_min_main_after = child.node.min_main_after(store, layout_type).to_px(parent_main, DEFAULT_MIN);
            let child_max_main_after = child.node.max_main_after(store, layout_type).to_px(parent_main, DEFAULT_MAX);

            main_axis.push(StretchItem::new(
                index,
                factor,
                ItemType::After,
                child_min_main_after,
                child_max_main_after,
            ));
        }

        loop {
            // If all stretch items are frozen, exit the loop.
            if main_axis.iter().all(|item| item.frozen) {
                break;
            }

            // Compute free space in the main axis.
            let child_main_free_space = parent_main - child.main_non_flex;

            // Total size violation in the main axis.
            let mut total_violation = 0.0;

            for item in main_axis.iter_mut().filter(|item| !item.frozen) {
                let actual_main = (item.factor * child_main_free_space / child.main_flex_sum).round();

                let clamped = actual_main.clamp(item.min, item.max);
                item.violation = clamped - actual_main;
                total_violation += item.violation;

                match item.item_type {
                    ItemType::Before => child.main_before = clamped,
                    ItemType::Size => child.main = clamped,
                    ItemType::After => child.main_after = clamped,
                }
            }

            for item in main_axis.iter_mut().filter(|item| !item.frozen) {
                // Freeze over-stretched items.
                item.frozen = match total_violation {
                    total if total > 0.0 => item.violation > 0.0,
                    total if total < 0.0 => item.violation < 0.0,
                    _ => true,
                };

                // If the item is frozen, adjust the used_space and sum of main stretch factors.
                if item.frozen {
                    child.main_flex_sum -= item.factor;
                }
            }
        }

        if let Stretch(_) = child_main {
            let child_size = layout(child.node, layout_type, child.main, child.cross, cache, tree, store);
            child.main = child_size.main;
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
                main_pos += child.main + child.main_after;
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
