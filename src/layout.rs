use smallvec::SmallVec;

use crate::{Alignment, Cache, CacheExt, LayoutType, Node, NodeExt, PositionType, Size, Units::*};

const DEFAULT_MIN: f32 = -f32::MAX;
const DEFAULT_MAX: f32 = f32::MAX;
const DEFAULT_BORDER_WIDTH: f32 = 0.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemType {
    Size,
    After,
}

/// Represents a space or size which has stretch units.
#[derive(Copy, Clone)]
struct StretchItem {
    // The child index of the item.
    index: usize,
    // The stretch factor of the item.
    factor: f32,
    // The type of stretch item, either space-before, size, or space-after.
    item_type: ItemType,
    // The violation of the stretch item after clamping.
    violation: f32,
    // The computed size of the stretch item.
    computed: f32,
    // Whether or not the stretch item is frozen.
    frozen: bool,
    // The minimum size of the stretch item.
    min: f32,
    // The maximum size of the stretch item.
    max: f32,
}

impl StretchItem {
    pub fn new(index: usize, factor: f32, item_type: ItemType, min: f32, max: f32) -> Self {
        Self { index, factor, item_type, violation: 0.0, computed: 0.0, frozen: false, min, max }
    }
}

#[derive(Debug, Copy, Clone)]
struct ChildNode<'a, N: Node> {
    // A reference to the node.
    node: &'a N,
    // Computed cross size of the node.
    cross: f32,
    // Computed main size of the node.
    main: f32,

    main_after: f32,
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
/// * `parent_main` - The size of the parent of the `node` on its main axis or the main-size of the node if the node is stretch (determined by parent).
/// * `parent_cross` - The size of the parent of the `node` on its cross axis or the cross-size of the node if the node is stretch (determined by parent).
/// * `cache` - A mutable reference to the [`Cache`].
/// * `tree` - A mutable reference to the [`Tree`](crate::Node::Tree).
/// * `store` - A mutable reference to the [`Store`](crate::Node::Store).
/// * `sublayout` - A mutable reference to the [`SubLayout`](crate::Node::SubLayout) context.
///
/// # Example
///
/// ```
/// layout(&root, LayoutType::Column, 600.0, 600.0, &mut cache, &tree, &store, &mut sublayout);
/// ```
#[allow(clippy::too_many_arguments)]
pub(crate) fn layout<N, C>(
    node: &N,
    parent_layout_type: LayoutType,
    parent_main: f32,
    parent_cross: f32,
    cache: &mut C,
    tree: &<N as Node>::Tree,
    store: &<N as Node>::Store,
    sublayout: &mut <N as Node>::SubLayout<'_>,
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

    let mut min_main = if main.is_stretch() {
        DEFAULT_MIN
    } else {
        node.min_main(store, parent_layout_type).to_px(parent_main, DEFAULT_MIN)
    };

    let mut max_main = if main.is_stretch() {
        DEFAULT_MAX
    } else {
        node.max_main(store, parent_layout_type).to_px(parent_main, DEFAULT_MAX)
    };

    let mut min_cross = node.min_cross(store, parent_layout_type).to_px(parent_cross, DEFAULT_MIN);

    let mut max_cross = node.max_cross(store, parent_layout_type).to_px(parent_cross, DEFAULT_MAX);

    // Compute main-axis size.
    let mut computed_main = match main {
        Pixels(val) => val,
        Percentage(val) => (parent_main * (val / 100.0)).round(),
        Stretch(_) => parent_main,
        Auto => 0.0,
    };

    // Compute cross-axis size.
    let mut computed_cross = match cross {
        Pixels(val) => val,
        Percentage(val) => (parent_cross * (val / 100.0)).round(),
        Stretch(_) => parent_cross,
        Auto => 0.0,
    };

    let border_main_before =
        node.border_main_before(store, parent_layout_type).to_px(computed_main, DEFAULT_BORDER_WIDTH);
    let border_main_after =
        node.border_main_after(store, parent_layout_type).to_px(computed_main, DEFAULT_BORDER_WIDTH);
    let border_cross_before =
        node.border_cross_before(store, parent_layout_type).to_px(computed_cross, DEFAULT_BORDER_WIDTH);
    let border_cross_after =
        node.border_cross_after(store, parent_layout_type).to_px(computed_cross, DEFAULT_BORDER_WIDTH);

    // Get the total number of children of the node.
    let num_children = node.children(tree).filter(|child| child.visible(store)).count();

    // Get the total number of relative children of the node.
    let num_parent_directed_children = node
        .children(tree)
        .filter(|child| child.position_type(store).unwrap_or_default() == PositionType::Relative)
        .filter(|child| child.visible(store))
        .count();

    // Apply content sizing.
    if (node.min_main(store, parent_layout_type).is_auto() || node.min_cross(store, parent_layout_type).is_auto())
        && num_parent_directed_children == 0
    {
        let p_main = if node.min_main(store, parent_layout_type).is_auto() { None } else { Some(computed_main) };
        let p_cross = if node.min_cross(store, parent_layout_type).is_auto() { None } else { Some(computed_cross) };

        if let Some(content_size) = node.content_sizing(store, sublayout, parent_layout_type, p_main, p_cross) {
            min_main = content_size.0;
            min_cross = content_size.1;
        }
    }

    if (main.is_auto() || cross.is_auto()) && num_parent_directed_children == 0 {
        let p_main = if main.is_auto() { None } else { Some(computed_main) };
        let p_cross = if cross.is_auto() { None } else { Some(computed_cross) };

        if let Some(content_size) = node.content_sizing(store, sublayout, parent_layout_type, p_main, p_cross) {
            computed_main = content_size.0;
            computed_cross = content_size.1;
        }
    }

    computed_main = computed_main.max(min_main).min(max_main);
    computed_cross = computed_cross.max(min_cross).min(max_cross);

    // Determine the parent_main/cross size to pass to the children based on the layout type of the parent and the node.
    // i.e. if the parent layout type and the node layout type are different, swap the main and the cross axes.
    let (mut parent_main, mut parent_cross) = if parent_layout_type == layout_type {
        (computed_main, computed_cross)
    } else {
        (computed_cross, computed_main)
    };

    // Sum of all space and size flex factors on the main-axis of the node.
    let mut main_flex_sum = 0.0;

    // List of child nodes for the current node.
    let mut children = SmallVec::<[ChildNode<N>; 32]>::with_capacity(num_children);

    // List of stretch nodes for the current node.
    // A stretch node is any flexible space/size. e.g. main_before, main, and main_after are separate stretch nodes
    let mut main_axis = SmallVec::<[StretchItem; 32]>::new();

    // Parent overrides for child auto space.
    let padding_main_before = node.padding_main_before(store, layout_type).to_px(parent_main, 0.0);
    let padding_main_after = node.padding_main_after(store, layout_type).to_px(parent_main, 0.0);
    let padding_cross_before = node.padding_cross_before(store, layout_type).to_px(parent_cross, 0.0);
    let padding_cross_after = node.padding_cross_after(store, layout_type).to_px(parent_cross, 0.0);

    let min_main_between = node.min_main_between(store, layout_type);
    let max_main_between = node.max_main_between(store, layout_type);

    parent_main = parent_main - padding_main_before - padding_main_after - border_main_before - border_main_after;
    parent_cross = parent_cross - padding_cross_before - padding_cross_after - border_cross_before - border_cross_after;

    // Determine index of first and last relative child nodes.
    let mut iter = node
        .children(tree)
        .filter(|child| child.visible(store))
        .filter(|child| child.position_type(store).unwrap_or_default() == PositionType::Relative)
        .enumerate();

    let first = iter.next().map(|(index, _)| index);
    let last = iter.last().map_or(first, |(index, _)| Some(index));

    let mut node_children = node
        .children(tree)
        .filter(|child| child.visible(store))
        .filter(|child| child.position_type(store).unwrap_or_default() == PositionType::Relative)
        .enumerate()
        .peekable();

    // Compute space and size of non-flexible relative children.
    while let Some((index, child)) = node_children.next() {
        let child_main = child.main(store, layout_type);
        let child_cross = child.cross(store, layout_type);

        // Get fixed-size constraints.
        let child_min_main = child.min_main(store, layout_type);
        let child_max_main = child.max_main(store, layout_type);

        let child_min_cross = child.min_cross(store, layout_type);
        let child_max_cross = child.max_cross(store, layout_type);

        let mut computed_child_main_after = 0.0f32;

        if last != Some(index) {
            let child_main_after = node.main_between(store, layout_type);

            if let Stretch(factor) = child_main_after {
                main_flex_sum += factor;
                main_axis.push(StretchItem::new(
                    index,
                    factor,
                    ItemType::After,
                    min_main_between.to_px(parent_main, DEFAULT_MIN),
                    max_main_between.to_px(parent_main, DEFAULT_MAX),
                ));
            } else {
                computed_child_main_after =
                child_main_after.to_px_clamped(parent_main, 0.0, min_main_between, max_main_between);
            }
        }
        
        let mut computed_child_main = 0.0;

        // Collect stretch main items.
        if let Stretch(factor) = child_main {
            main_flex_sum += factor;
            main_axis.push(StretchItem::new(
                index,
                factor,
                ItemType::Size,
                child_min_main.to_px(parent_main, DEFAULT_MIN),
                child_max_main.to_px(parent_main, DEFAULT_MAX),
            ));
        } else {
            computed_child_main = child_main.to_px_clamped(parent_cross, 0.0, child_min_main, child_max_main);
        }

        let mut computed_child_cross = child_cross.to_px_clamped(parent_cross, 0.0, child_min_cross, child_max_cross);
        

        // Compute fixed-size child main and cross.
        if !child_main.is_stretch() && (!child_cross.is_stretch() || child_min_cross.is_auto()) {
            let child_size = layout(child, layout_type, parent_main, parent_cross, cache, tree, store, sublayout);

            computed_child_main = child_size.main;
            computed_child_cross = child_size.cross;
        }

        children.push(ChildNode {
            node: child,
            cross: computed_child_cross,
            main: computed_child_main,
            main_after: computed_child_main_after,
        });
    }

    // Sum of all child nodes on the main-axis.
    let mut main_sum: f32 = children.iter().map(|child| child.main + child.main_after).sum();

    // Maximum of all child nodes on the cross-axis.
    let mut cross_max: f32 = children.iter().map(|child| child.cross).reduce(f32::max).unwrap_or_default();

    // Determine auto main and cross size from space and size of children.
    if num_parent_directed_children != 0 {
        if main.is_auto() || node.min_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else {
                min_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if node.max_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && main_sum != 0.0 {
                max_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else if cross_max != 0.0 {
                max_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if cross.is_auto() || node.min_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else {
                min_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }

        if node.max_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && cross_max != 0.0 {
                max_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else if main_sum != 0.0 {
                max_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }
    }

    computed_main = computed_main.max(min_main).min(max_main);
    computed_cross = computed_cross.max(min_cross).min(max_cross);

    let (mut parent_main, mut parent_cross) = if parent_layout_type == layout_type {
        (computed_main, computed_cross)
    } else {
        (computed_cross, computed_main)
    };

    parent_main = parent_main - padding_main_before - padding_main_after - border_main_before - border_main_after;
    parent_cross = parent_cross - padding_cross_before - padding_cross_after - border_cross_before - border_cross_after;

    // Compute stretch size on the cross-axis for relative children.
    for child in children
        .iter_mut()
        .filter(|child| child.node.position_type(store).unwrap_or_default() == PositionType::Relative)
        .filter(|child| child.node.cross(store, layout_type).is_stretch())
    {
        if !child.node.main(store, layout_type).is_stretch() {
            let child_size = layout(child.node, layout_type, parent_main, parent_cross, cache, tree, store, sublayout);
            child.main = child_size.main;
            child.cross = child_size.cross;
        } else {
            let child_min_cross = if child.node.min_cross(store, layout_type).is_auto() {
                child.cross
            } else {
                child.node.min_cross(store, layout_type).to_px(parent_cross, DEFAULT_MIN)
            };

            let child_max_cross = child.node.max_cross(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

            child.cross = parent_cross.clamp(child_min_cross, child_max_cross);
        }
    }

    main_sum = children.iter().map(|child| child.main + child.main_after).sum();
    cross_max = children.iter().map(|child| child.cross).reduce(f32::max).unwrap_or_default();

    // Determine auto main and cross size from space and size of children.
    if num_parent_directed_children != 0 {
        if main.is_auto() || node.min_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else {
                min_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if node.max_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && main_sum != 0.0 {
                max_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else if cross_max != 0.0 {
                max_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if cross.is_auto() || node.min_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else {
                min_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }

        if node.max_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && cross_max != 0.0 {
                max_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else if main_sum != 0.0 {
                max_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }
    }

    computed_main = computed_main.max(min_main).min(max_main);
    computed_cross = computed_cross.max(min_cross).min(max_cross);

    // Compute flexible space and size on the main axis for relative children.
    if !main_axis.is_empty() {
        loop {
            // If all stretch items are frozen, exit the loop.
            if main_axis.iter().all(|item| item.frozen) {
                break;
            }

            // Calculate free space on the main-axis.
            let free_main_space = parent_main - main_sum;

            let mut total_violation = 0.0;

            for item in main_axis.iter_mut().filter(|item| !item.frozen) {
                let mut actual_main = (item.factor * free_main_space / main_flex_sum).round();

                let child = &mut children[item.index];

                if item.item_type == ItemType::Size {
                    let child_size = layout(
                        child.node,
                        layout_type,
                        actual_main,
                        if child.node.cross(store, layout_type).is_stretch() { child.cross } else { parent_cross },
                        cache,
                        tree,
                        store,
                        sublayout,
                    );
                    child.cross = child_size.cross;
                    actual_main = child_size.main;

                    if child.node.min_main(store, layout_type).is_auto() {
                        item.min = child_size.main;
                    }
                }

                let clamped = actual_main.min(item.max).max(item.min);
                item.violation = clamped - actual_main;
                total_violation += item.violation;
                item.computed = clamped;
            }

            for item in main_axis.iter_mut().filter(|item| !item.frozen) {
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

                    match item.item_type {
                        ItemType::Size => {
                            child.main = item.computed;
                        }

                        ItemType::After => {
                            child.main_after = item.computed;
                        }
                    }

                    main_sum = children.iter().map(|child| child.main + child.main_after).sum();
                }
            }
        }
    }

    main_sum = children.iter().map(|child| child.main + child.main_after).sum();
    cross_max = children.iter().map(|child| child.cross).reduce(f32::max).unwrap_or_default();

    // Determine auto main and cross size from space and size of children.
    if num_parent_directed_children != 0 {
        if main.is_auto() || node.min_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else {
                min_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if node.max_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && main_sum != 0.0 {
                max_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else if cross_max != 0.0 {
                max_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if cross.is_auto() || node.min_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else {
                min_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }

        if node.max_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && cross_max != 0.0 {
                max_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else if main_sum != 0.0 {
                max_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }
    }

    computed_main = computed_main.max(min_main).min(max_main);
    computed_cross = computed_cross.max(min_cross).min(max_cross);

    let (mut parent_main, mut parent_cross) = if parent_layout_type == layout_type {
        (computed_main, computed_cross)
    } else {
        (computed_cross, computed_main)
    };

    parent_main = parent_main - padding_main_before - padding_main_after - border_main_before - border_main_after;
    parent_cross = parent_cross - padding_cross_before - padding_cross_after - border_cross_before - border_cross_after;

    for child in children
        .iter_mut()
        .filter(|child| child.node.position_type(store).unwrap_or_default() == PositionType::Relative)
        .filter(|child| child.node.cross(store, layout_type).is_stretch())
    {
        let child_min_cross = if child.node.min_cross(store, layout_type).is_auto() {
            child.cross
        } else {
            child.node.min_cross(store, layout_type).to_px(parent_cross, DEFAULT_MIN)
        };

        let child_max_cross = child.node.max_cross(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        child.cross = parent_cross.clamp(child_min_cross, child_max_cross);
    }

    // Absolute Children

    let node_children = node
        .children(tree)
        .filter(|child| child.position_type(store).unwrap_or_default() == PositionType::Absolute)
        .filter(|child| child.visible(store));

    // Compute space and size of non-flexible absolute children.
    for child in node_children {
        let main = if child.main(store, layout_type).is_stretch() {
            let child_min_main = child.min_main(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
            let child_max_main = child.max_main(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

            let child_main_before = child.main_before(store, layout_type).to_px(parent_main, 0.0);
            let child_main_after = child.main_after(store, layout_type).to_px(parent_main, 0.0);

            parent_main.clamp(child_min_main, child_max_main) - child_main_before - child_main_after
        } else {
            parent_main
        };

        let cross = if child.cross(store, layout_type).is_stretch() {
            let child_min_cross = child.min_cross(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
            let child_max_cross = child.max_cross(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

            let child_cross_before = child.cross_before(store, layout_type).to_px(parent_main, 0.0);
            let child_cross_after = child.cross_after(store, layout_type).to_px(parent_main, 0.0);

            parent_cross.clamp(child_min_cross, child_max_cross) - child_cross_before - child_cross_after
        } else {
            parent_cross
        };

        let child_size = layout(child, layout_type, main, cross, cache, tree, store, sublayout);

        let computed_child_main = child_size.main;
        let computed_child_cross = child_size.cross;

        children.push(ChildNode {
            node: child,
            cross: computed_child_cross,
            main: computed_child_main,
            main_after: 0.0,
        });
    }

    let alignment = node.alignment(store).unwrap_or_default();

    // Set size and position of children in the cache.
    let mut main_pos = padding_main_before + border_main_before;
    for child in children.iter() {
        let child_position = child.node.position_type(store).unwrap_or_default();

        match child_position {
            PositionType::Absolute => {
                let child_main_before = child.node.main_before(store, layout_type);
                let child_main_after = child.node.main_after(store, layout_type);
                let child_cross_before = child.node.cross_before(store, layout_type);
                let child_cross_after = child.node.cross_after(store, layout_type);

                let parent_main = parent_main + padding_main_before + padding_main_after;
                let parent_cross = parent_cross + padding_cross_before + padding_cross_after;

                let child_main_pos = match (child_main_before, child_main_after) {
                    (Pixels(val), _) => val,
                    (Percentage(val), _) => val * 0.01 * parent_main,
                    (_, Pixels(val)) => parent_main - val - child.main,
                    (_, Percentage(val)) => parent_main - child.main - val * 0.01 * parent_main,
                    (Stretch(b), Stretch(a)) => {
                        if b == a {
                            (parent_main - child.main) * 0.5
                        } else {
                            (parent_main - child.main) * (b / (b + a))
                        }
                    }
                    (Stretch(_), Auto) => parent_main - child.main,
                    (Auto, Stretch(_)) => 0.0,
                    (Auto, Auto) => 0.0,
                };

                let child_cross_pos = match (child_cross_before, child_cross_after) {
                    (Pixels(val), _) => val,
                    (Percentage(val), _) => val * 0.01 * parent_cross,
                    (_, Pixels(val)) => parent_cross - val - child.cross,
                    (_, Percentage(val)) => parent_cross - child.cross - val * 0.01 * parent_cross,
                    (Stretch(b), Stretch(a)) => {
                        if b == a {
                            (parent_cross - child.cross) * 0.5
                        } else {
                            (parent_cross - child.cross) * (b / (b + a))
                        }
                    }
                    (Stretch(_), Auto) => parent_cross - child.cross,
                    (Auto, Stretch(_)) => 0.0,
                    (Auto, Auto) => 0.0,
                };

                cache.set_rect(
                    child.node,
                    layout_type,
                    child_main_pos + border_main_before,
                    child_cross_pos + border_cross_before,
                    child.main,
                    child.cross,
                );
            }

            PositionType::Relative => {
                let (mut child_main_pos, mut child_cross_pos) = match alignment {
                    Alignment::TopLeft => (0.0, 0.0),
                    Alignment::TopCenter => (0.0, 0.5),
                    Alignment::TopRight => (0.0, 1.0),
                    Alignment::Left => (0.5, 0.0),
                    Alignment::Center => (0.5, 0.5),
                    Alignment::Right => (0.5, 1.0),
                    Alignment::BottomLeft => (1.0, 0.0),
                    Alignment::BottomCenter => (1.0, 0.5),
                    Alignment::BottomRight => (1.0, 1.0),
                };

                if layout_type == LayoutType::Row {
                    std::mem::swap(&mut child_main_pos, &mut child_cross_pos);
                }

                child_main_pos *= parent_main - main_sum;
                child_cross_pos *= parent_cross - child.cross;

                if let Some(main_scroll) = node.main_scroll(store, layout_type) {
                    child_main_pos = main_scroll
                }

                if let Some(cross_scroll) = node.cross_scroll(store, layout_type) {
                    child_cross_pos = cross_scroll
                }

                cache.set_rect(
                    child.node,
                    layout_type,
                    main_pos + child_main_pos,
                    child_cross_pos + padding_cross_before + border_cross_before,
                    child.main,
                    child.cross,
                );
                main_pos += child.main + child.main_after;
            }
        };
    }

    // Return the computed size, propagating it back up the tree.
    Size { main: computed_main, cross: computed_cross }
}
