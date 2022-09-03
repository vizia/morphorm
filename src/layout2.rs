use std::marker::PhantomData;

use smallvec::SmallVec;

use crate::Units::*;
use crate::{Cache, LayoutType, Node, Units};

#[derive(Debug, Copy, Clone)]
pub struct BoxConstraints {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

impl Default for BoxConstraints {
    fn default() -> Self {
        BoxConstraints { min: (0.0, 0.0), max: (0.0, 0.0) }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub main: f32,
    pub cross: f32,
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    MainBefore,
    Main,
    MainAfter,
    Cross,
}

#[derive(Copy, Clone)]
pub struct StretchNode<'a, 'b, N: Node<'b>> {
    node: &'a N,

    value: f32,
    min: f32,
    max: f32,
    axis: Axis,
    p: PhantomData<&'b N>,
}

// Perform layout on a node
pub fn layout<'a, N, C>(
    node: &N,
    parent_layout_type: LayoutType,
    bc: &BoxConstraints,
    cache: &mut C,
    tree: &'a <N as Node<'a>>::Tree,
    store: &'a <N as Node<'a>>::Store,
) -> Size
where
    N: Node<'a>,
    C: Cache<Node = N::CacheKey>,
{
    // TODO: Investigate whether a box constraints struct is needed. So far only the parent main/cross is needed,
    // which is currently stored in bc.0.max and bc.1.max respectively. It's possible that the other constraints will
    // be needed when min/max sized are added so I've left it fo now.

    // NOTE: Due to the recursive nature of this function, any code written before the loop on the children is performed
    // on the 'down' pass of the tree, and any code after the loop is performed on the 'up' phase.
    // However, positioning of children need to happen after all children have a computed size, so it's placed after the loop
    // causing the positioning to occur on the 'up' phase.
    // This has the effect of positioning children relative to the parent and not absolutely. To account for this, the system in charge
    // of rendering the nodes must also recursively traverse the tree and add the parent position to the node position.
    // Unclear whether morphorm should provide that or whether the user should do that. At the moment it's on the user.
    // See draw_node() in 'examples/common/mod.rs'.

    // FIXME: The main_sum and cross_sum values don't take into account the flex lines.
    // TODO: Percentage size for non-flex children
    // TODO: Finish main_before, add main_after, cross_before, cross_after
    // TODO: Add parent space overrides
    // TODO: Absolute positioning
    // TODO: Min/Max constraints for space and size
    // TODO: ADD TESTS FOR EVERYTHING!

    let layout_type = node.layout_type(store).unwrap_or_default();

    let main = node.main(store).unwrap_or(Units::Stretch(1.0));
    let cross = node.cross(store).unwrap_or(Units::Stretch(1.0));

    let mut computed_main = 0.0;
    let mut computed_cross = 0.0;

    // Compute fixed-size main size
    match main {
        Pixels(val) => {
            computed_main = val;
        }

        Percentage(val) => {
            computed_main = (bc.max.0 * (val / 100.0)).round();
        }

        Stretch(_) => {
            computed_main = bc.max.0;
        }

        _ => {}
    }

    // Compute fixed-size cross size
    match cross {
        Pixels(val) => {
            computed_cross = val;
        }

        Percentage(val) => {
            computed_cross = (bc.max.1 * (val / 100.0)).round();
        }

        Stretch(_) => {
            if bc.max.1.is_finite() {
                computed_cross = bc.max.1;
            }
        }

        _ => {}
    }

    // Apply content-size
    match (parent_layout_type, layout_type) {
        (LayoutType::Row, LayoutType::Column) if main == Units::Auto => {
            if let Some(content_size) = node.content_size(store, computed_cross) {
                computed_main = content_size;
            }
        }

        (LayoutType::Row, LayoutType::Row) if cross == Units::Auto => {
            if let Some(content_size) = node.content_size(store, computed_main) {
                computed_cross = content_size
            }
        }

        (LayoutType::Column, LayoutType::Row) if main == Units::Auto => {
            if let Some(content_size) = node.content_size(store, computed_cross) {
                computed_main = content_size
            }
        }

        (LayoutType::Column, LayoutType::Column) if cross == Units::Auto => {
            if let Some(content_size) = node.content_size(store, computed_main) {
                computed_cross = content_size;
            }
        }

        _ => {}
    }

    let mut flex_lines = Vec::new();

    let mut main_sum = 0.0f32;
    let mut cross_max = 0.0f32;

    // Sum of all non-flexible space/size on the main-axis of the current flex line
    let mut main_non_flex = 0.0f32;
    // Max of all non-flexible space/size on the cross-axis of the current flex line
    let mut cross_non_flex = 0.0f32;
    // Sum of all space/size flex factors on the main-axis of the current flex line
    let mut main_flex_sum = 0.0;
    // TODO: Should children & stretch_nodes be merged? It would require branching to filter stretch nodes
    // when computing stretch space/size and children when positioning nodes.
    // List of child nodes for the current line
    let mut children = Vec::new();
    // List of stretch nodes for the current flex line
    // A stretch node is any flexible space/size. e.g. main_before, main, and main_after are separate stretch nodes
    let mut stretch_nodes = SmallVec::<[StretchNode<N>; 3]>::new();

    // Sum of all non-flexible space/size on the cross axis
    let mut cross_sum = 0.0;

    let mut cross_flex_sum = 0.0f32;

    // Keeps track of whether there is a node with stretch cross size somewhere in the flex line.
    // This is then added to the cross_sum when the line wraps
    let mut cross_flex = 0.0;

    // Compute non-flexible children
    for child in node.children(tree) {
        let child_main_before = child.main_before(store).unwrap_or(Units::Auto);
        let child_main = child.main(store).unwrap_or(Units::Stretch(1.0));

        match child_main_before {
            Pixels(val) => {
                main_sum += val;
                main_non_flex += val;
                cache.set_main_before(child.key(), val);
            }

            Stretch(factor) => {
                main_flex_sum += factor;

                // Add node to list of stretch nodes for the line
                stretch_nodes.push(StretchNode {
                    node: child,
                    value: factor,
                    min: 0.0,
                    max: std::f32::INFINITY,
                    axis: Axis::MainBefore,
                    p: PhantomData::default(),
                });
            }

            _ => {}
        }

        let child_cross = child.cross(store).unwrap_or(Units::Stretch(1.0));

        match child_main {
            Pixels(val) => {
                // If the main_sum exceeds the parent width then add the node to the next flex line
                if main_non_flex + val > computed_main && !children.is_empty() {
                    flex_lines.push((
                        children.clone(),
                        stretch_nodes.clone(),
                        main_non_flex,
                        main_flex_sum,
                        cross_non_flex,
                    ));
                    children.clear();
                    stretch_nodes.clear();

                    cross_sum += cross_non_flex;
                    cross_flex_sum += cross_flex;

                    // Reset cross flex for next line
                    cross_flex = 0.0;

                    main_non_flex = 0.0;
                    cross_non_flex = 0.0;
                    main_flex_sum = 0.0;
                }

                // Compute child box constraints
                let child_bc = BoxConstraints { min: (0.0, 0.0), max: (val, 0.0) };

                let child_size = layout(child, layout_type, &child_bc, cache, tree, store);

                main_sum += child_size.main;
                cross_max = cross_max.max(child_size.cross);
                main_non_flex += child_size.main;
                cross_non_flex = cross_non_flex.max(child_size.cross);

                children.push(child);
            }

            Stretch(factor) => {
                main_flex_sum += factor;

                // Add node to list of stretch nodes for the line
                stretch_nodes.push(StretchNode {
                    node: child,
                    value: factor,
                    min: 0.0,
                    max: std::f32::INFINITY,
                    axis: Axis::Main,
                    p: PhantomData::default(),
                });

                children.push(child);
            }

            Auto => {
                let child_bc =
                    BoxConstraints { min: (0.0, 0.0), max: (computed_main, computed_cross) };

                let child_size = layout(child, layout_type, &child_bc, cache, tree, store);

                main_non_flex += child_size.main;
                cross_non_flex += child_size.cross;

                main_sum += child_size.main;
                cross_max = cross_max.max(child_size.cross);

                children.push(child);

                // TODO: Is this in the correct place?
                // If the main_sum exceeds the parent width then add the node to the next flex line
                if main_non_flex > computed_main {
                    flex_lines.push((
                        children.clone(),
                        stretch_nodes.clone(),
                        main_non_flex,
                        main_flex_sum,
                        cross_non_flex,
                    ));
                    children.clear();
                    stretch_nodes.clear();
                    main_non_flex = 0.0;
                    cross_non_flex = 0.0;
                    main_flex_sum = 0.0;
                    cross_sum += cross_non_flex;
                }
            }

            _ => {}
        }

        match child_cross {
            Stretch(_) => {
                cross_flex = 1.0;
            }

            _ => {}
        }
    }

    // Increment the cross_flx_sum if there's a flex cross node on the last line
    cross_flex_sum += cross_flex;
    cross_sum += cross_non_flex;

    // Finished iterating children so push the last line onto the stack
    flex_lines.push((
        children.clone(),
        stretch_nodes.clone(),
        main_non_flex,
        main_flex_sum,
        cross_non_flex,
    ));

    let mut free_cross_space = (computed_cross - cross_sum).max(0.0);

    let mut cross_used_space = 0.0;
    let num_of_flex_lines = flex_lines.len();
    // Compute flexible space/size
    for (_, axis, main_non_flex, main_flex_sum, cross_non_flex) in flex_lines.iter_mut() {
        // Calculate free space for the current flex line
        let free_main_space = (computed_main - *main_non_flex).max(0.0);
        let mut remainder: f32 = 0.0;
        let main_px_per_flex = free_main_space / *main_flex_sum;

        for item in axis.iter() {
            let factor = item.value;
            let desired_main = factor * main_px_per_flex + remainder;
            let actual_main = desired_main.round();
            remainder = desired_main - actual_main;

            match item.axis {
                Axis::MainBefore => {
                    cache.set_main_before(item.node.key(), actual_main);
                }

                Axis::MainAfter => {
                    cache.set_main_after(item.node.key(), actual_main);
                }

                Axis::Main => {
                    let child_bc = BoxConstraints {
                        min: (actual_main, free_cross_space),
                        max: (actual_main, free_cross_space),
                    };

                    let child_size = layout(item.node, layout_type, &child_bc, cache, tree, store);

                    if child_size.main.is_finite() {
                        main_sum += child_size.main;
                        cross_max = cross_max.max(child_size.cross);
                    } else {
                        println!("WARNING: Flex child in Auto parent");
                    }

                    // If the content-size produces a higher cross_non_flex than it was before then the free_cross_space needs to be adjusted
                    if child_size.cross > *cross_non_flex {
                        let diff = child_size.cross - *cross_non_flex;
                        free_cross_space -= diff;
                        *cross_non_flex = child_size.cross;
                    }
                }

                _ => {}
            }
        }
    }

    // The layout computation for stretch main (above) may cause content-size to change the cross-non-flex of the flex line.
    // Which affects the free_cross_space, therefore the iteration on stretch cross size needs to be done after all the stretch main calculations.
    for (i, (children, _, _, _, cross_non_flex)) in flex_lines.iter_mut().enumerate() {
        let flex_line_cross_space = if num_of_flex_lines == (i + 1) {
            // Last flex line so use the space left for the line height
            (computed_cross - cross_used_space).max(0.0)
        } else {
            if *cross_non_flex == 0.0 {
                // No non-stretch cross nodes so use distribution of free space
                free_cross_space / cross_flex_sum.max(1.0)
            } else {
                // There's a non-stretch node in the line so use its cross size
                *cross_non_flex
            }
        };

        // Need to iterate on children with stretch cross space here because the flex line height needs to be calculated
        // first, which is done above.
        for child in children.iter() {
            let child_cross = child.cross(store).unwrap_or(Stretch(1.0));

            match child_cross {
                Stretch(_) => {
                    // This is probably a bad idea but the thought is that stretch nodes on the cross axis
                    // can only be the full height of the flex line at this point. So instead of calling `layout`
                    // again we just set the node cross in cache directly.
                    match layout_type {
                        LayoutType::Row => {
                            cache.set_height(child.key(), flex_line_cross_space);
                        }

                        LayoutType::Column => {
                            cache.set_width(child.key(), flex_line_cross_space);
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        *cross_non_flex = flex_line_cross_space;
        cross_used_space += *cross_non_flex;
    }

    // Position children
    let parent_posx = cache.posx(node.key());
    let parent_posy = cache.posx(node.key());

    let mut cross_pos = 0.0;
    for (children, _, _, _, cross_size) in flex_lines.iter() {
        let mut main_pos = 0.0;
        for child in children.iter() {
            let main_before = cache.main_before(child.key());

            main_pos += main_before;

            match layout_type {
                LayoutType::Row => {
                    cache.set_posx(child.key(), parent_posx + main_pos);
                    cache.set_posy(child.key(), parent_posy + cross_pos);
                    let child_width = cache.width(child.key());
                    main_pos += child_width;
                }

                LayoutType::Column => {
                    cache.set_posy(child.key(), parent_posy + main_pos);
                    cache.set_posx(child.key(), parent_posx + cross_pos);
                    let child_height = cache.height(child.key());
                    main_pos += child_height;
                }

                _ => {}
            }
        }
        cross_pos += cross_size;
    }

    // Constrain the computed size to the sum/max of the children.
    // This is also how content-size gets propagated up the tree.
    // TODO: Constrain to min/max size when added
    // TODO: This won't work if the nodes have wrapped. In that case the sum of the longest flex line should be used.
    computed_main = computed_main.max(main_sum);
    computed_cross = computed_cross.max(cross_max);

    match parent_layout_type {
        LayoutType::Row => {
            cache.set_width(node.key(), computed_main);
            cache.set_height(node.key(), computed_cross);
        }

        LayoutType::Column => {
            cache.set_height(node.key(), computed_main);
            cache.set_width(node.key(), computed_cross);
        }

        _ => {}
    }

    // Propagate the computed size back up the tree
    Size { main: computed_main, cross: computed_cross }
}
