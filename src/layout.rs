use smallvec::SmallVec;

use crate::{Cache, LayoutType, Node, NodeExt, Units, PositionType};
use crate::Units::*;

const DEFAULT_MIN: f32 = -f32::MAX;
const DEFAULT_MAX: f32 = f32::MAX;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub main: f32,
    pub cross: f32,
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    MainBefore,
    Main,
    MainAfter,
}

#[derive(Copy, Clone)]
pub struct StretchNode<'a, N: Node> {
    // A reference to the node.
    node: &'a N,
    // The index of the node.
    index: usize,
    // The stretch factor of the node. 
    factor: f32,
    // The minimum constraint of the node.
    _min: f32,
    // The maximum constraint of the node.
    _max: f32,

    axis: Axis,
}

#[derive(Copy, Clone)]
pub struct ChildNode<'a, N: Node> {
    node: &'a N,

    // Sum of the flex factors on the main axis of the node.
    main_flex_sum: f32,

    // The available free space on the main axis of the node.
    main_non_flex: f32,

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

// Perform layout on a node
pub fn layout<N, C>(
    node: &N,
    parent_layout_type: LayoutType,
    parent_main: f32,
    node_cross: f32,
    cache: &mut C,
    tree: &<N as Node>::Tree,
    store: &<N as Node>::Store,
) -> Size
where
    N: Node,
    C: Cache<Node = N::CacheKey>,
{
    // NOTE: Due to the recursive nature of this function, any code written before the loop on the children is performed
    // on the 'down' pass of the tree, and any code after the loop is performed on the 'up' phase.
    // However, positioning of children need to happen after all children have a computed size, so it's placed after the loop
    // causing the positioning to occur on the 'up' phase.
    // This has the effect of positioning children relative to the parent and not absolutely. To account for this, the system in charge
    // of rendering the nodes must also recursively traverse the tree and add the parent position to the node position.
    // Unclear whether morphorm should provide that or whether the user should do that. At the moment it's on the user.
    // See draw_node() in 'examples/common/mod.rs'.

    // TODO: Min/Max constraints for space and size

    // The layout type of the node. Determines the main and cross axes of the children.
    let layout_type = node.layout_type(store).unwrap_or_default();

    // The desired main-axis size of the node.
    let main = node.main(store, parent_layout_type).unwrap_or(Units::Stretch(1.0));
    // The desired cross-axis size of the node.
    let cross = node.cross(store, parent_layout_type).unwrap_or(Units::Stretch(1.0));

    // Compute main-axis size.
    let mut computed_main = match main {
        Pixels(val) => val,

        Percentage(val) => (parent_main * (val / 100.0)).round(),

        Stretch(_) => parent_main,

        Auto => 0.0,
    };

    let min_main = node.min_main(store, parent_layout_type).unwrap_or_default().to_px(parent_main, DEFAULT_MIN);
    let max_main = node.max_main(store, parent_layout_type).unwrap_or_default().to_px(parent_main, DEFAULT_MAX);

    // Apply main-axis size constraints for pixels and percentage.
    computed_main = computed_main.clamp(min_main, max_main);

    // Cross-axis size is determined by the parent.
    let mut computed_cross = node_cross;

    // Apply content size
    if main == Units::Auto && cross != Units::Auto {
        if let Some(content_size) = node.content_size(store, computed_cross) {
            computed_main = content_size;
        }
    }

    // If the cross-size is supposed to be determined by the parent then this should probably be done by the parent as well
    if cross == Units::Auto && main != Units::Auto {
        if let Some(content_size) = node.content_size(store, computed_main) {
            computed_cross = content_size;
        }
    }

    // Determine the main/cross size for the children based on the layout type of the parent and the node.
    // i.e. if the parent layout type and the node layout type are different, swap the main and the cross
    let (parent_main, parent_cross) = if parent_layout_type != layout_type {
        (computed_cross, computed_main)
    } else {
        (computed_main, computed_cross)
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
    let node_child_main_before = node.child_main_before(store, layout_type).unwrap_or_default();
    let node_child_main_after = node.child_main_after(store, layout_type).unwrap_or_default();
    let node_child_cross_before = node.child_cross_before(store, layout_type).unwrap_or_default();
    let node_child_cross_after = node.child_cross_after(store, layout_type).unwrap_or_default();
    let node_child_main_between = node.main_between(store, layout_type).unwrap_or_default();

    // Determine index of first and last parent-directed child nodes.
    let mut iter = node
        .children(tree)
        .enumerate()
        .filter(|(_, child)| child.position_type(store).unwrap_or_default() == PositionType::ParentDirected);

    let first = iter.next().map(|(index, _)| index);
    let last = iter.last().map_or(first, |(index, _)| Some(index));

    let num_children = node.children(tree).count();

    // Compute non-flexible children.
    for (index, child) in node.children(tree).enumerate() {
        let child_position_type = child.position_type(store).unwrap_or_default();

        let mut child_main_before = child.main_before(store, layout_type).unwrap_or_default();
        let child_main = child.main(store, layout_type).unwrap_or(Units::Stretch(1.0));
        let mut child_main_after = child.main_after(store, layout_type).unwrap_or_default();

        let mut child_cross_before = child.cross_before(store, layout_type).unwrap_or_default();
        let child_cross = child.cross(store, layout_type).unwrap_or(Units::Stretch(1.0));
        let mut child_cross_after = child.cross_after(store, layout_type).unwrap_or_default();

        // Apply parent overrides to auto child space.
        if child_main_before == Units::Auto {
            if first == Some(index) || child_position_type == PositionType::SelfDirected {
                child_main_before = node_child_main_before;
            } else {
                child_main_before = node_child_main_between;
            }
        }

        if child_main_after == Units::Auto {
            if last == Some(index) || child_position_type == PositionType::SelfDirected {
                child_main_after = node_child_main_after;
            }
        }

        if child_cross_before == Units::Auto {
            child_cross_before = node_child_cross_before;
        }

        if child_cross_after == Units::Auto {
            child_cross_after = node_child_cross_after;
        }

        // Sum of flex factors on the main-axis of the child node.
        let mut child_main_flex_sum = 0.0;
        // Sum of flex factors on the cross-axis of the child node.
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

        let child_min_cross_before =
            child.min_cross_before(store, layout_type).unwrap_or_default().to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross_before =
            child.max_cross_before(store, layout_type).unwrap_or_default().to_px(parent_cross, DEFAULT_MAX);

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

        let child_min_cross_after =
            child.min_cross_after(store, layout_type).unwrap_or_default().to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross_after =
            child.max_cross_after(store, layout_type).unwrap_or_default().to_px(parent_cross, DEFAULT_MAX);

        computed_child_cross_after = computed_child_cross_after.clamp(child_min_cross_after, child_max_cross_after);

        match child_cross {
            Pixels(val) => {
                computed_child_cross = val;
            }

            Percentage(val) => {
                computed_child_cross = (parent_cross * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_cross_flex_sum += factor;
            }

            _ => {}
        }

        // Apply cross-axis size constraints for pixels & percentage.
        let child_min_cross = child.min_cross(store, layout_type).unwrap_or_default().to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross = child.max_cross(store, layout_type).unwrap_or_default().to_px(parent_cross, DEFAULT_MAX);

        computed_child_cross = computed_child_cross.clamp(child_min_cross, child_max_cross);

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
                    _min: 0.0,
                    _max: std::f32::INFINITY,
                    axis: Axis::MainBefore,
                });
            }

            _ => {}
        }

        let child_min_main_before =
            child.min_main_before(store, layout_type).unwrap_or_default().to_px(parent_main, DEFAULT_MIN);
        let child_max_main_before =
            child.max_main_before(store, layout_type).unwrap_or_default().to_px(parent_main, DEFAULT_MAX);

        computed_child_main_before = computed_child_main_before.clamp(child_min_main_before, child_max_main_before);

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
                    _min: 0.0,
                    _max: std::f32::INFINITY,
                    axis: Axis::MainAfter,
                });
            }

            _ => {}
        }

        let child_min_main_after =
            child.min_main_after(store, layout_type).unwrap_or_default().to_px(parent_main, DEFAULT_MIN);
        let child_max_main_after =
            child.max_main_after(store, layout_type).unwrap_or_default().to_px(parent_main, DEFAULT_MAX);

        computed_child_main_after = computed_child_main_after.clamp(child_min_main_after, child_max_main_after);

        // Total computed size on the cross-axis of the child.
        let mut child_cross_non_flex = computed_child_cross_before + computed_child_cross + computed_child_cross_after;

        match child_main {
            Stretch(factor) => {
                child_main_flex_sum += factor;

                // Add node to list of stretch nodes for the node
                stretch_nodes.push(StretchNode {
                    node: child,
                    index,
                    factor,
                    _min: 0.0,
                    _max: std::f32::INFINITY,
                    axis: Axis::Main,
                });
            }

            _ => {

                let child_size = layout(child, layout_type, parent_main, computed_child_cross, cache, tree, store);

                computed_child_main = child_size.main;
                computed_child_cross = child_size.cross;

                if child_cross == Units::Auto {
                    child_cross_non_flex += computed_child_cross;
                }
            }
        }

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
    // This is the computed main-axis size of the node minus the sum of the main-axis sizes of all the children.
    let free_main_space = (parent_main.max(main_sum) - main_non_flex).max(0.0);
    let mut remainder: f32 = 0.0;
    let main_px_per_flex = free_main_space / main_flex_sum;

    // Compute flexible space and size on the main axis
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
                let child_size = layout(item.node, layout_type, actual_main, computed_child_cross, cache, tree, store);

                children[item.index].cross_non_flex += child_size.cross;
                cross_max = cross_max.max(children[item.index].cross_non_flex);
            }
        }
    }

    // Compute flexible space and size on the cross-axis.
    // This needs to be done after computing the main-axis because layout computation for stretch children may cause
    // the cross space to change due to content size.
    // Hmmm, but surely this only applies to auto cross size?
    for child in children.iter_mut() {
        let child_cross_free_space = parent_cross.max(cross_max) - child.cross_non_flex;
        // println!("{:?} {:?} child_cross_free_space: {} {} {}", node.key(), child.node.key(), parent_cross, cross_max, child.cross_non_flex);
        let cross_px_per_flex = child_cross_free_space / child.cross_flex_sum;

        let child_cross_before = child.node.cross_before(store, layout_type).unwrap_or(Auto);
        let child_cross = child.node.cross(store, layout_type).unwrap_or(Stretch(1.0));
        let child_cross_after = child.node.cross_after(store, layout_type).unwrap_or(Auto);

        match child_cross_before {
            Stretch(factor) => {
                let desired_cross = factor * cross_px_per_flex + child.cross_remainder;
                let actual_cross = desired_cross.round();
                child.cross_remainder = desired_cross - actual_cross;
                child.cross_before = actual_cross;
            }

            _ => {}
        }

        match child_cross {
            Stretch(factor) => {
                // TODO: remove duplication
                let desired_cross = factor * cross_px_per_flex + child.cross_remainder;
                let actual_cross = desired_cross.round();
                child.cross_remainder = desired_cross - actual_cross;

                // At this stage stretch nodes on the cross-axis can only be the determined size so we can set it directly
                // in the cache without needing to call layout again.
                match layout_type {
                    LayoutType::Row => {
                        cache.set_height(child.node.key(), actual_cross);
                    }

                    LayoutType::Column => {
                        cache.set_width(child.node.key(), actual_cross);
                    }
                }
            }

            _ => {}
        }

        match child_cross_after {
            Stretch(factor) => {
                let desired_cross = factor * cross_px_per_flex + child.cross_remainder;
                let actual_cross = desired_cross.round();
                child.cross_remainder = desired_cross - actual_cross;
                child.cross_after = actual_cross;
            }

            _ => {}
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
