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

        Auto => {
            computed_main = std::f32::INFINITY;
        }
    }

    // Compute fixed-size cross size
    match cross {
        Pixels(val) => {
            computed_cross = val;
        }

        Percentage(val) => {
            computed_cross = (bc.max.1 * (val / 100.0)).round();
            println!("computed cross: {:?} {} {}", bc, val, computed_cross);
        }

        Stretch(_) => {
            computed_cross = bc.max.1;
        }

        _ => {}
    }

    // Max of the sum of main space/size of the flex lines
    let mut main_max = 0.0f32;
    // Sum of the max of cross space/size of the flex lines
    let mut cross_sum = 0.0;

    // Apply content-size
    // TODO: Detect here if the user is trying to use content-size with wrapping on the same axis.
    // TODO: Should this only apply if the node has no children?
    match (parent_layout_type, layout_type) {
        (LayoutType::Row, LayoutType::Column) if main == Units::Auto => {
            if let Some(content_size) = node.content_size(store, computed_cross) {
                computed_main = content_size;
                println!("Row Column - {}", content_size);
            }
        }

        (LayoutType::Row, LayoutType::Row) if cross == Units::Auto => {
            if let Some(content_size) = node.content_size(store, computed_main) {
                computed_cross = content_size;
                println!("Row Row - {}", content_size);
            }
        }

        (LayoutType::Column, LayoutType::Row) if main == Units::Auto => {
            if let Some(content_size) = node.content_size(store, computed_cross) {
                computed_main = content_size;
                println!("Column Row - {}", content_size);
            }
        }

        (LayoutType::Column, LayoutType::Column) if cross == Units::Auto => {
            if let Some(content_size) = node.content_size(store, computed_main) {
                computed_cross = content_size;
                println!("Column Column - {}", content_size);
            }
        }

        _ => {}
    }


    let mut flex_lines = Vec::new();

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

    // Sum of all space/size flex factors across all flex lines
    let mut cross_flex_sum = 0.0f32;

    // Keeps track of whether there is a node with stretch cross size somewhere in the flex line.
    // This is then added to the cross_sum when the line wraps
    let mut cross_flex = 0.0;

    let mut has_children = false;

    // Compute non-flexible children
    for child in node.children(tree) {
        
        has_children = true;

        // Sum of flex factors for stretch cross_before, cross, and cross_after
        let mut child_cross_flex_sum = 0.0;

        // Sum of non-flex cross_before, cross, and cross_after
        let mut child_cross_non_flex = 0.0;

        let child_main_before = child.main_before(store).unwrap_or(Units::Auto);
        let child_main = child.main(store).unwrap_or(Units::Stretch(1.0));
        let child_main_after = child.main_after(store).unwrap_or(Units::Auto);

        let mut computed_child_main_before = 0.0;
        let mut computed_child_main = 0.0;
        let mut computed_child_main_after = 0.0;

        let mut computed_child_cross_before = 0.0;
        let mut computed_child_cross = 0.0;
        let mut computed_child_cross_after = 0.0;

        let child_cross_before = child.cross_before(store).unwrap_or(Units::Auto);
        let child_cross = child.cross(store).unwrap_or(Units::Stretch(1.0));
        let child_cross_after = child.cross_after(store).unwrap_or(Units::Auto);

        match child_cross_before {
            Pixels(val) => {
                computed_child_cross_before = val;
            }

            Percentage(val) => {
                computed_child_cross_before = (computed_cross * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_cross_flex_sum += factor;
                // what to do here?
            }

            _=> {}
        }

        match child_cross_after {
            Pixels(val) => {
                computed_child_cross_after = val;
            }

            Percentage(val) => {
                computed_child_cross_after = (computed_cross * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_cross_flex_sum += factor;
                // what to do here?
            }

            _=> {}
        }

        match child_main_before {
            Pixels(val) => {
                computed_child_main_before = val;
                // main_sum += val;
                // main_non_flex += val;
                //cache.set_main_before(child.key(), val);
            }

            Percentage(val) => {
                computed_child_main_before = (computed_main * (val / 100.0)).round();
                //main_non_flex += computed_main_before;
                //cache.set_main_before(child.key(), computed_main_before);
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

        match child_main_after {
            Pixels(val) => {
                // main_sum += val;
                // main_non_flex += val;
                // cache.set_main_after(child.key(), val);
                computed_child_main_after = val;
            }

            Percentage(val) => {
                // let computed_main_after = (computed_main * (val / 100.0)).round();
                // main_non_flex += computed_main_after;
                // cache.set_main_after(child.key(), computed_main_after);
                computed_child_main_after = (computed_main * (val / 100.0)).round();
            }

            Stretch(factor) => {
                main_flex_sum += factor;

                // Add node to list of stretch nodes for the line
                stretch_nodes.push(StretchNode {
                    node: child,
                    value: factor,
                    min: 0.0,
                    max: std::f32::INFINITY,
                    axis: Axis::MainAfter,
                    p: PhantomData::default(),
                });
            }

            _ => {}
        }

        match child_main {
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
            }

            _ => {
                // This is required because bc.max.1 is being used for both the 'parent size' and the 'desired stretch size'.
                // This could be avoided by explicitly passing the parent size to layout.
                let bc_cross = if let Units::Stretch(_) = child_cross {
                    0.0
                } else {
                    computed_cross
                };

                let child_bc =
                    BoxConstraints { min: (0.0, 0.0), max: (computed_main, bc_cross) };

                let child_size = layout(child, layout_type, &child_bc, cache, tree, store);

                computed_child_main = child_size.main;
                computed_child_cross = child_size.cross;
            }
        }

        let child_main_sum = computed_child_main_before + computed_child_main + computed_child_main_after;
        let child_cross_sum = computed_child_cross_before + computed_child_cross + computed_child_cross_after;

        if main_non_flex + child_main_sum > computed_main && !children.is_empty() {
            flex_lines.push((
                children.clone(),
                stretch_nodes.clone(),
                main_non_flex,
                main_flex_sum,
                cross_non_flex,
            ));
            children.clear();
            stretch_nodes.clear();

            cross_flex_sum += cross_flex;

            println!("{:?} {:?} cross_non_flex: {}", node.key(), child.key(), cross_non_flex);  
            cross_sum += cross_non_flex;
            // Reset cross flex for next line
            cross_flex = 0.0;
            
            main_non_flex = 0.0;
            cross_non_flex = 0.0;
            main_flex_sum = 0.0;
            
        }
        

        main_non_flex += child_main_sum;
        cross_non_flex = cross_non_flex.max(child_cross_sum);
        //cross_sum += cross_non_flex;
        main_max = main_max.max(main_non_flex);
        
        
        children.push((child, child_cross_flex_sum, child_cross_sum));

        cache.set_main_before(child.key(), computed_child_main_before);
        cache.set_main_after(child.key(), computed_child_main_after);
        cache.set_cross_before(child.key(), computed_child_cross_before);
        cache.set_cross_after(child.key(), computed_child_cross_after);

        // This needs to go here (after computing if the line should wrap) because the line wrapping resets the cross_flex value.
        

        match child_cross {
            Stretch(factor) => {
                cross_flex = 1.0;
                child_cross_flex_sum += factor;
            }

            _ => {}
        }
    }

    // Increment the cross_flex_sum if there's a flex cross node on the last line
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

    println!("{:?} flex lines: {}  free_space: {} {} {}  cross_flex_sum: {}", node.key(), flex_lines.len(), free_cross_space, computed_cross, cross_sum, cross_flex_sum);


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
                        //main_sum += child_size.main;
                        //cross_max = cross_max.max(child_size.cross);
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

    let mut cross_used_space = 0.0;
    let num_of_flex_lines = flex_lines.len();
    // The layout computation for stretch main (above) may cause content-size to change the cross-non-flex of the flex line.
    // Which affects the free_cross_space, therefore the iteration on stretch cross size needs to be done after all the stretch main calculations.
    for (i, (children, _, _, _, cross_non_flex)) in flex_lines.iter_mut().enumerate() {
        let flex_line_cross_space = if num_of_flex_lines == (i + 1) {
            println!("A: {}", cross_used_space);
            // Last flex line so use the space left for the line height
            (computed_cross - cross_used_space).max(0.0)
        } else {
            if *cross_non_flex == 0.0 {
                // No non-stretch cross nodes so use distribution of free space
                println!("B: {}", cross_flex_sum);
                free_cross_space / cross_flex_sum.max(1.0)
            } else {
                println!("C: {}", cross_non_flex);
                // There's a non-stretch node in the line so use its cross size
                *cross_non_flex
            }
        };

        println!("{:?} flex_line_cross_space {}", node.key(), flex_line_cross_space);

        // Need to iterate on children with stretch cross space here because the flex line height needs to be calculated
        // first, which is done above.
        for (child, child_cross_flex_sum, child_cross_non_flex) in children.iter() {

            let free_cross_space = flex_line_cross_space - child_cross_non_flex;
            let mut remainder: f32 = 0.0;
            let cross_px_per_flex = free_cross_space / (*child_cross_flex_sum).max(1.0);

            let child_cross_before = child.cross_before(store).unwrap_or(Auto);

            match child_cross_before {
                Stretch(factor) => {
                    let desired_cross = factor * cross_px_per_flex + remainder;
                    let actual_cross = desired_cross.round();
                    remainder = desired_cross - actual_cross;
                    // println!("{:?} actual_cross: {}", child.key(), actual_cross);
                    cache.set_cross_before(child.key(), actual_cross);
                }

                _=> {}
            }

            let child_cross_after = child.cross_after(store).unwrap_or(Auto);

            match child_cross_after {
                Stretch(factor) => {
                    let desired_cross = factor * cross_px_per_flex + remainder;
                    let actual_cross = desired_cross.round();
                    remainder = desired_cross - actual_cross;
                    cache.set_cross_after(child.key(), actual_cross);
                }

                _=> {}
            }

            let child_cross = child.cross(store).unwrap_or(Stretch(1.0));

            match child_cross {
                Stretch(factor) => {

                    // TODO: remove duplication
                    let desired_cross = factor * cross_px_per_flex + remainder;
                    let actual_cross = desired_cross.round();
                    remainder = desired_cross - actual_cross;

                    //println!("{:?} actual_cross: {} free_cross_space: {}", node.key(), actual_cross, free_cross_space);
                    //let child_bc =
                    //BoxConstraints { min: (actual_cross, free_cross_space), max: (actual_cross, free_cross_space) };

                    //let child_size = layout(*child, layout_type, &child_bc, cache, tree, store);


                    // This is probably a bad idea but the thought is that stretch nodes on the cross axis
                    // can only be the full height of the flex line at this point. So instead of calling `layout`
                    // again we just set the node cross in cache directly.
                    match layout_type {
                        LayoutType::Row => {
                            cache.set_height(child.key(), actual_cross);
                        }

                        LayoutType::Column => {
                            cache.set_width(child.key(), actual_cross);
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
        for (child, _, _) in children.iter() {
            let main_before = cache.main_before(child.key());
            let main_after = cache.main_after(child.key());
            let cross_before = cache.cross_before(child.key());
            let cross_after = cache.cross_after(child.key());

            main_pos += main_before;
            // cross_pos += cross_before;

            match layout_type {
                LayoutType::Row => {
                    cache.set_posx(child.key(), parent_posx + main_pos);
                    cache.set_posy(child.key(), parent_posy + cross_pos + cross_before);
                    let child_width = cache.width(child.key());
                    main_pos += child_width;
                }

                LayoutType::Column => {
                    cache.set_posy(child.key(), parent_posy + main_pos);
                    cache.set_posx(child.key(), parent_posx + cross_pos + cross_before);
                    let child_height = cache.height(child.key());
                    main_pos += child_height;
                }

                _ => {}
            }

            main_pos += main_after;
        }
        cross_pos += cross_size;
    }

    // Constrain the computed size to the sum/max of the children.
    // This is also how content-size gets propagated up the tree.
    // TODO: Constrain to min/max size when added
    // TODO: This won't work if the nodes have wrapped. In that case the sum of the longest flex line should be used.
    
    println!("{:?} : computed_main {}  computed_cross: {}", node.key(), computed_main, computed_cross);
    // computed_main = computed_main.max(main_max);
    // computed_cross = computed_cross.max(cross_sum);

    // This part is required for auto size when the node has children but conflicts with the content size when the node doesn't have children
    // TODO: Make it so a node can only have content size if it has no children?
    if has_children {
        if parent_layout_type == layout_type {
            if let Auto = main {
                computed_main = main_max;
            }
    
            if let Auto = cross {
                computed_cross = cross_sum;
            }
        } else {
            if let Auto = main {
                computed_main = cross_sum;
            }
    
            if let Auto = cross {
                computed_cross = main_max;
            }
        }
    }




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
