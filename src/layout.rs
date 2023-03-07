// Temp
#![allow(dead_code)]

use smallvec::SmallVec;

use crate::Units::*;
use crate::{Cache, LayoutType, Node, NodeExt, PositionType, Units};

const DEFAULT_MIN: f32 = -f32::MAX;
const DEFAULT_MAX: f32 = f32::MAX;

/// A type which represents the computed size of a node after [`layout`].
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Size {
    /// The computed size on the main axis.
    pub main: f32,
    /// The computed size on the cross axis.
    pub cross: f32,
}

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
    // The minimum constraint of the node.
    min: f32,
    // The maximum constraint of the node.
    max: f32,

    axis: Axis,
}

#[derive(Debug, Copy, Clone)]
struct ChildNode<'a, N: Node> {
    // A reference to the node.
    node: &'a N,
    // Sum of the flex factors on the main axis of the node.
    main_flex_sum: f32,
    // The available free space on the main axis of the node.
    main_non_flex: f32,
    // A remainder used during stretch computation.
    main_remainder: f32,
    // Sum of the cross_before, cross, and cross_after flex factors of the node.
    cross_flex_sum: f32,

    cross_non_flex: f32,

    cross: f32,

    cross_remainder: f32,

    // Computed main-before space of the node.
    main_before: f32,
    // Computed main-after space of the node.
    main_after: f32,
    // Computed cross-before space of the node.
    cross_before: f32,
    // Computed cross-after space of the node.
    cross_after: f32,
}

/// Performs layout on a tree of nodes starting from a root node.
/// 
/// The algorithm recurses down the tree, in depth-first order, and performs
/// layout on every node starting from the input `node`.
///
/// # Arguments
/// 
/// * `node` - Root node to start layout from.
/// * `parent_layout_type` - The [`LayoutType`] of the parent of the `node`. If the `node` has no parent then pass `None`.
/// * `parent_main` - The size of the parent of the `node` on its main axis. If the `node` has no parent then pass `None`.
/// * `parent_cross` - The size of the parent of the `node` along its cross axis. If the `node` has no parent then pass `None`.
/// * `cache` - A mutable reference to the [`Cache`].
/// * `tree` - A mutable reference to the [`Tree`](crate::Node::Tree).
/// * `store` - A mutable reference to the [`Store`](crate::Node::Store).
/// 
/// # Example
/// 
/// ```
/// layout(&root, None, None, None, &mut cache, &tree, &store);
/// ```
pub fn layout<N, C>(
    node: &N,
    parent_layout_type: Option<LayoutType>,
    parent_main: Option<f32>,
    parent_cross: Option<f32>,
    cache: &mut C,
    tree: &<N as Node>::Tree,
    store: &<N as Node>::Store,
) -> Size
where
    N: Node,
    C: Cache<CacheKey = N::CacheKey>,
{
    // NOTE: Due to the recursive nature of this function, any code written before the loop on the children is performed
    // on the 'down' pass of the tree, and any code after the loop is performed on the 'up' phase.
    // However, positioning of children need to happen after all children have a computed size, so it's placed after the loop
    // causing the positioning to occur on the 'up' phase.
    // This has the effect of positioning children relative to the parent and not absolutely. To account for this, the system in charge
    // of rendering the nodes must also recursively traverse the tree and add the parent position to the node position.
    // Unclear whether morphorm should provide that or whether the user should do that. At the moment it's on the user.
    // See draw_node() in 'examples/common/mod.rs'.

    // TODO: Min/Max constraints for stretch space and size



    // If `None` then `node` is a root node and the parent layout type is not important.
    let parent_layout_type = parent_layout_type.unwrap_or_default();

    // The layout type of the node. Determines the main and cross axes of the children.
    let layout_type = node.layout_type(store).unwrap_or_default();

    // The desired main-axis and cross-axis sizes of the node.
    let main = node.main(store, parent_layout_type);
    let cross = node.cross(store, parent_layout_type);

    // If the `node` is a root node then use its main as the parent_main.
    let parent_main = if let Some(pm) = parent_main {
        pm
    } else if let Pixels(val) = main {
        val
    } else {
        panic!("Root node must have pixels main size");
    };

    // If the `node` is a root node then use its cross as the parent_cross.
    let parent_cross = if let Some(pc) = parent_cross {
        pc
    } else if let Pixels(val) = cross {
        val
    } else {
        panic!("Root node must have pixels cross size");
    };

    // Compute main-axis size.
    let mut computed_main = match main {
        Pixels(val) => val,
        Percentage(val) => (parent_main * (val / 100.0)).round(),
        Stretch(_) => parent_main,
        Auto => 0.0,
    };

    // Cross-axis size is determined by the parent.
    let mut computed_cross = match cross {
        Pixels(val) => val,
        Percentage(val) => (parent_cross * (val / 100.0)).round(),
        Stretch(_) => parent_cross,
        Auto => 0.0,
    };

    // Apply content size to main axis.
    if main == Units::Auto && cross != Units::Auto {
        if let Some(content_size) = node.content_size(store, computed_cross) {
            computed_main = content_size;
        }
    }

    // Apply content size to cross axis.
    if cross == Units::Auto && main != Units::Auto {
        if let Some(content_size) = node.content_size(store, computed_main) {
            computed_cross = content_size;
        }
    }

    // Apply main-axis size constraints for pixels and percentage.
    let min_main = node.min_main(store, parent_layout_type).to_px(parent_main, DEFAULT_MIN);
    let max_main = node.max_main(store, parent_layout_type).to_px(parent_main, DEFAULT_MAX);
    computed_main = computed_main.clamp(min_main, max_main);

    // Apply cross-axis size constraints for pixels and percentage.
    let min_cross = node.min_cross(store, parent_layout_type).to_px(parent_cross, DEFAULT_MIN);
    let max_cross = node.max_cross(store, parent_layout_type).to_px(parent_cross, DEFAULT_MAX);
    computed_cross = computed_cross.clamp(min_cross, max_cross);

    // Determine the parent_main/cross size to pass to the children based on the layout type of the parent and the node.
    // i.e. if the parent layout type and the node layout type are different, swap the main and the cross axes.
    let (parent_main, parent_cross) = if parent_layout_type == layout_type {
        (computed_main, computed_cross)
    } else {
        (computed_cross, computed_main)
    };

    // Sum of all non-flexible space and size on the main-axis of the node.
    let mut main_non_flex = 0.0f32;

    // Sum of all space and size flex factors on the main-axis of the node.
    let mut main_flex_sum = 0.0;

    // Sum of all child nodes on the main-axis.
    let mut main_sum = 0.0f32;

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

    let num_children = node.children(tree).count();

    // Compute space and size of non-flexible children.
    for (index, child) in node.children(tree).enumerate() {
        let child_position_type = child.position_type(store).unwrap_or_default();

        let mut child_main_before = child.main_before(store, layout_type);
        let child_main = child.main(store, layout_type);
        let mut child_main_after = child.main_after(store, layout_type);

        let mut child_cross_before = child.cross_before(store, layout_type);
        let child_cross = child.cross(store, layout_type);
        let mut child_cross_after = child.cross_after(store, layout_type);

        // Apply parent overrides to auto child space.
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

        // Sum of flex factors on the main and cross axes of the child node.
        let mut child_main_flex_sum = 0.0;
        let mut child_cross_flex_sum = 0.0;


        let mut computed_child_main_before = 0.0;
        let mut computed_child_main = 0.0;
        let mut computed_child_main_after = 0.0;

        let mut computed_child_cross_before = 0.0;
        let mut computed_child_cross = 0.0;
        let mut computed_child_cross_after = 0.0;

        match child_cross_before {
            Pixels(val) => {
                computed_child_cross_before = val;
            }

            Percentage(val) => {
                computed_child_cross_before = (parent_cross * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_cross_flex_sum += factor;
            }

            _ => {}
        }

        // Apply constraints to child cross_before for pixels and percentage.
        let child_min_cross_before = child.min_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross_before = child.max_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        computed_child_cross_before = computed_child_cross_before.clamp(child_min_cross_before, child_max_cross_before);

        match child_cross_after {
            Pixels(val) => {
                computed_child_cross_after = val;
            }

            Percentage(val) => {
                computed_child_cross_after = (parent_cross * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_cross_flex_sum += factor;
            }

            _ => {}
        }

        // Apply constraints to child cross_after for pixels and percentage.
        let child_min_cross_after = child.min_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross_after = child.max_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        computed_child_cross_after = computed_child_cross_after.clamp(child_min_cross_after, child_max_cross_after);


        let child_min_main_before = child.min_main_before(store, layout_type).to_px(parent_main, DEFAULT_MIN);
        let child_max_main_before = child.max_main_before(store, layout_type).to_px(parent_main, DEFAULT_MAX);

        match child_main_before {
            Pixels(val) => {
                computed_child_main_before = val;
            }

            Percentage(val) => {
                computed_child_main_before = (parent_main * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_main_flex_sum += factor;

                stretch_nodes.push(StretchNode {
                    node: child,
                    index,
                    factor,
                    min: child_min_main_before,
                    max: child_max_main_before,
                    axis: Axis::MainBefore,
                });
            }

            _ => {}
        }

        // Apply constraints to child main_before for pixels and percentage.
        computed_child_main_before = computed_child_main_before.clamp(child_min_main_before, child_max_main_before);

        let child_min_main_after = child.min_main_after(store, layout_type).to_px(parent_main, DEFAULT_MIN);
        let child_max_main_after = child.max_main_after(store, layout_type).to_px(parent_main, DEFAULT_MAX);

        match child_main_after {
            Pixels(val) => {
                computed_child_main_after = val;
            }

            Percentage(val) => {
                computed_child_main_after = (parent_main * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_main_flex_sum += factor;

                stretch_nodes.push(StretchNode {
                    node: child,
                    index,
                    factor,
                    min: child_min_main_after,
                    max: child_max_main_after,
                    axis: Axis::MainAfter,
                });
            }

            _ => {}
        }

        // Apply constraints to child main_after for pixels and percentage.
        computed_child_main_after = computed_child_main_after.clamp(child_min_main_after, child_max_main_after);

        match (child_main, child_cross) {
            (Stretch(main_factor), Stretch(cross_factor)) => {
                child_main_flex_sum += main_factor;
                child_cross_flex_sum += cross_factor;

                stretch_nodes.push(StretchNode {
                    node: child,
                    index,
                    factor: main_factor,
                    min: DEFAULT_MIN,
                    max: DEFAULT_MAX,
                    axis: Axis::Main,
                })
            },

            (Stretch(main_factor), _) => {
                child_main_flex_sum += main_factor;

                stretch_nodes.push(StretchNode {
                    node: child,
                    index,
                    factor: main_factor,
                    min: DEFAULT_MIN,
                    max: DEFAULT_MAX,
                    axis: Axis::Main,
                });

                stretch_nodes.rotate_right(1);
            },

            (_, Stretch(cross_factor)) => {
                child_cross_flex_sum += cross_factor;
            }

            _=> {
                let child_size = layout(child, Some(layout_type), Some(parent_main), Some(parent_cross), cache, tree, store);

                computed_child_main = child_size.main;
                computed_child_cross = child_size.cross;
            }
        }

        // Total computed size on the cross-axis of the child.
        let child_cross_non_flex = computed_child_cross_before + computed_child_cross + computed_child_cross_after;

        // Total computed size on the main-axis of the child.
        let child_main_non_flex = computed_child_main_before + computed_child_main + computed_child_main_after;

        if child_position_type == PositionType::ParentDirected {
            main_non_flex += child_main_non_flex;
            main_flex_sum += child_main_flex_sum;

            main_sum += child_main_non_flex;
        } else {
            main_sum = main_sum.max(child_main_non_flex);
        }

        cross_max = cross_max.max(child_cross_non_flex);

        children.push(ChildNode {
            node: child,
            main_flex_sum: child_main_flex_sum,
            main_non_flex: child_main_non_flex,
            main_remainder: 0.0,
            cross_flex_sum: child_cross_flex_sum,
            cross_non_flex: child_cross_non_flex,
            cross: computed_child_cross,
            cross_remainder: 0.0,
            main_before: computed_child_main_before,
            main_after: computed_child_main_after,
            cross_before: computed_child_cross_before,
            cross_after: computed_child_cross_after,
        });
    }

    // Calculate free space on the main-axis for the node.
    let free_main_space = (parent_main.max(main_sum) - main_non_flex).max(0.0);
    let mut remainder: f32 = 0.0;
    let main_px_per_flex = free_main_space / main_flex_sum;

    // Compute flexible space and size on the main axis.
    for item in stretch_nodes.iter() {
        let child_position_type = item.node.position_type(store).unwrap_or_default();

        let actual_main = if child_position_type == PositionType::SelfDirected {
            let child_main_free_space = (parent_main.max(main_sum) - children[item.index].main_non_flex).max(0.0);
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

        let computed_child_cross = children[item.index].cross;

        match item.axis {
            Axis::MainBefore => {
                children[item.index].main_before = actual_main;
            }

            Axis::MainAfter => {
                children[item.index].main_after = actual_main;
            }

            Axis::Main => {
                let child_cross = item.node.cross(store, layout_type);
                let child_size = match child_cross {
                    Stretch(_) => {
                        layout(item.node, Some(layout_type), Some(actual_main), Some(computed_child_cross), cache, tree, store)
                    }

                    _=> {
                        layout(item.node, Some(layout_type), Some(actual_main), Some(computed_cross), cache, tree, store)
                    }
                };

                children[item.index].cross_non_flex += child_size.cross;
                cross_max = cross_max.max(children[item.index].cross_non_flex);
            }
        }
    }

    // Compute flexible space and size on the cross-axis.
    // This needs to be done after computing the main-axis because layout computation for stretch children may cause
    // the cross space to change due to content size.
    for child in children.iter_mut() {
        let child_cross_free_space = parent_cross.max(cross_max) - child.cross_non_flex;
        let cross_px_per_flex = child_cross_free_space / child.cross_flex_sum;

        let child_cross_before = child.node.cross_before(store, layout_type);
        let child_cross = child.node.cross(store, layout_type);
        let child_cross_after = child.node.cross_after(store, layout_type);

        if let Stretch(factor) = child_cross_before {
            let desired_cross = factor * cross_px_per_flex + child.cross_remainder;
            let actual_cross = desired_cross.round();
            child.cross_remainder = desired_cross - actual_cross;
            child.cross_before = actual_cross;
        }

        if let Stretch(factor) = child_cross {
            let desired_cross = factor * cross_px_per_flex + child.cross_remainder;
            let actual_cross = desired_cross.round();
            child.cross_remainder = desired_cross - actual_cross;

            let size = layout(child.node, Some(layout_type), Some(computed_main), Some(actual_cross), cache, tree, store);
            cross_max = cross_max.max(size.cross);
            main_sum += size.main;
        }

        if let Stretch(factor) = child_cross_after {
            let desired_cross = factor * cross_px_per_flex + child.cross_remainder;
            let actual_cross = desired_cross.round();
            child.cross_remainder = desired_cross - actual_cross;
            child.cross_after = actual_cross;
        }
    }

    // Position children.
    let mut main_pos = 0.0;
    for child in children.iter() {
        let child_position_type = child.node.position_type(store).unwrap_or_default();

        match child_position_type {
            PositionType::SelfDirected => match layout_type {
                LayoutType::Row => {
                    cache.set_posx(child.node.key(), child.main_before);
                    cache.set_posy(child.node.key(), child.cross_before);
                }

                LayoutType::Column => {
                    cache.set_posy(child.node.key(), child.main_before);
                    cache.set_posx(child.node.key(), child.cross_before);
                }
            },

            PositionType::ParentDirected => {
                main_pos += child.main_before;

                match layout_type {
                    LayoutType::Row => {
                        cache.set_posx(child.node.key(), main_pos);
                        cache.set_posy(child.node.key(), child.cross_before);
                        let child_width = cache.width(child.node.key());
                        main_pos += child_width;
                    }

                    LayoutType::Column => {
                        cache.set_posy(child.node.key(), main_pos);
                        cache.set_posx(child.node.key(), child.cross_before);
                        let child_height = cache.height(child.node.key());
                        main_pos += child_height;
                    }
                }

                main_pos += child.main_after;
            }
        };
    }

    // This part is required for auto size when the node has children but conflicts with the content size when the node doesn't have children
    // TODO: Make it so a node can only have content size if it has no children?
    // TODO: Potentially split and move this to before stretch calculations.
    if num_children != 0 {
        if parent_layout_type == layout_type {
            if let Auto = main {
                computed_main = main_sum;
            }

            if let Auto = cross {
                computed_cross = cross_max;
            }
        } else {
            if let Auto = main {
                computed_main = cross_max;
            }

            if let Auto = cross {
                computed_cross = main_sum;
            }
        }
    }

    // Set the computed size of the node in the cache.
    match parent_layout_type {
        LayoutType::Row => {
            cache.set_width(node.key(), computed_main);
            cache.set_height(node.key(), computed_cross);
        }

        LayoutType::Column => {
            cache.set_height(node.key(), computed_main);
            cache.set_width(node.key(), computed_cross);
        }
    }

    // Return the computed size, propagating it back up the tree.
    Size { main: computed_main, cross: computed_cross }
}