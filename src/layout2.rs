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
    Before,
    Size,
    After,
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

    //println!("layout: {:?} bc: {:?}", node.key(), bc);
    let layout_type = node.layout_type(store).unwrap_or_default();

    let main_before = node.main_before(store).unwrap_or(Units::Auto);
    let main = node.main(store).unwrap_or(Units::Stretch(1.0));
    let cross = node.cross(store).unwrap_or(Units::Stretch(1.0));

    let mut computed_main_before = 0.0;
    let mut computed_main = 0.0;
    let mut computed_cross = 0.0;


    match main_before {
        Pixels(val) => {
            computed_main_before = val;
        }

        _=> {}
    }

    //println!("computed main before: {:?} {}", node.key(), computed_main_before);

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

        _=> {}
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

        _=> {}
    }

    //println!("Entity: {:?}  Computed Width: {}", node.key(), computed_width);


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

    // Compute non-flexible children
    for child in node.children(tree) {
        let child_main_before = child.main_before(store).unwrap_or(Units::Auto);
        let child_main = child.main(store).unwrap_or(Units::Stretch(1.0));


        match child_main_before {
            Pixels(val) => {
                main_sum += val;
                main_non_flex += val;
                cache.set_main_before(child.key(), val);

                // // If the main_sum exceeds the parent width then add the node to the next flex line
                // if main_non_flex + val > computed_main {
                //     //println!("push line: {:?}", child.key());
                //     flex_lines.push((flex_line.clone(), main_non_flex + val, cross_non_flex));
                //     flex_line.clear();
                //     main_non_flex = 0.0;
                //     cross_non_flex = 0.0;
                // }

            }

            Stretch(factor) => {
                main_flex_sum += factor;
                
                // Add node to list of stretch nodes for the line
                stretch_nodes.push(StretchNode{
                    node: child,
                    value: factor,
                    min: 0.0,
                    max: std::f32::INFINITY,
                    axis: Axis::Before,
                    p: PhantomData::default(),
                });
            }

            _=> {}
        }

        match child_main {
            Pixels(val) => {
                // Compute child box constraints
                let child_bc = BoxConstraints {
                    min: (0.0, 0.0),
                    max: (val, computed_cross),
                };

                let child_size = layout(child, layout_type, &child_bc, cache, tree, store);

                

                main_sum += child_size.main;
                cross_max = cross_max.max(child_size.cross);
                main_non_flex += child_size.main;
                cross_non_flex = cross_non_flex.max(child_size.cross);

                children.push(child);

                // If the main_sum exceeds the parent width then add the node to the next flex line
                if main_non_flex > computed_main {
                    flex_lines.push((children.clone(), stretch_nodes.clone(), main_non_flex, main_flex_sum, cross_non_flex));
                    children.clear();
                    stretch_nodes.clear();
                    main_non_flex = 0.0;
                    cross_non_flex = 0.0;
                    main_flex_sum = 0.0;
                }
                
                // println!("set cross_non_flex to {}", child_size.cross);

            }

            Stretch(factor) => {
                main_flex_sum += factor;
                
                // Add node to list of stretch nodes for the line
                stretch_nodes.push(StretchNode{
                    node: child,
                    value: factor,
                    min: 0.0,
                    max: std::f32::INFINITY,
                    axis: Axis::Size,
                    p: PhantomData::default(),
                });
                
                children.push(child);
            },

            Auto => {

                let child_bc = BoxConstraints {
                    min: (0.0, 0.0),
                    max: (computed_main, computed_cross),
                };

                let child_size = layout(child, layout_type, &child_bc, cache, tree, store);
                
                main_non_flex += child_size.main;
                cross_non_flex += child_size.cross;

                main_sum += child_size.main;
                cross_max = cross_max.max(child_size.cross);

                children.push(child);

                // If the main_sum exceeds the parent width then add the node to the next flex line
                if main_non_flex > computed_main {
                    flex_lines.push((children.clone(), stretch_nodes.clone(), main_non_flex, main_flex_sum, cross_non_flex));
                    children.clear();
                    stretch_nodes.clear();
                    main_non_flex = 0.0;
                    cross_non_flex = 0.0;
                    main_flex_sum = 0.0;
                }
            }

            _ => {}
        }
    }

    flex_lines.push((children.clone(), stretch_nodes.clone(), main_non_flex, main_flex_sum, cross_non_flex));

    // Compute flexible space/size
    for (i, (_, axis, main_non_flex, mai_flex_sum, cross_non_flex)) in flex_lines.iter().enumerate() {
        // Calculate free space for the current flex line
        let free_main_space = (computed_main - main_non_flex).max(0.0);
        
        let mut remainder: f32 = 0.0;
    
        let mut main_flex: f32 = 0.0;
        let main_px_per_flex = free_main_space / main_flex_sum;
        //println!("{:?} line {} mai_per_flex {}", node.key(), i, main_px_per_flex);

        for item in axis.iter() {
            let factor = item.value;
            let desired_main = factor * main_px_per_flex + remainder;
            let actual_main = desired_main.round();
            remainder = desired_main - actual_main;

            //println!("{:?} actual main {}", item.node.key(), actual_main);
            
            match item.axis {
                Axis::Before => {
                    //println!("{:?} set before: {}", item.node.key(), actual_main);
                    cache.set_main_before(item.node.key(), actual_main);
                }

                Axis::After => {
                    cache.set_main_after(item.node.key(), actual_main);
                }

                Axis::Size => {
                    let child_bc =
                    BoxConstraints { min: (actual_main, computed_cross), max: (actual_main, computed_cross) };
    
                    let child_size = layout(item.node, layout_type, &child_bc, cache, tree, store);
        
                    if child_size.main.is_finite() {
                        main_sum += child_size.main;
                        cross_max = cross_max.max(child_size.cross);
                    } else {
                        println!("WARNING: Flex child in Auto parent");
                    }
                }
            }
        }
    }
    // for child in node.children(tree) {
    //     let child_main = child.main(store).unwrap_or(Units::Stretch(1.0));

    //     match child_main {
    //         Stretch(factor) => {
    //             let desired_main = factor * main_px_per_flex + remainder;
    //             let actual_main = desired_main.round();
    //             remainder = desired_main - actual_main;

    //             let child_bc =
    //                 BoxConstraints { min: (actual_main, computed_cross), max: (actual_main, computed_cross) };

    //             let child_size = layout(&child, layout_type, &child_bc, cache, tree, store);

    //             if child_size.main.is_finite() {
    //                 main_sum += child_size.main;
    //                 cross_max = cross_max.max(child_size.cross);
    //             } else {
    //                 println!("WARNING: Flex child in Auto parent");
    //             }
    //         }

    //         _ => {}
    //     }
    // }

    // Position children
    let parent_posx = cache.posx(node.key());
    let parent_posy = cache.posx(node.key());

    let mut cross_pos = 0.0;
    for (i, (children, _, _, _, cross_size)) in flex_lines.iter().enumerate() {
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

                _=> {}
            }
        }
        // TODO - add the height of the flex line
        cross_pos += cross_size;
        //main_pos = 0.0; // I can't remember why this is here or what it does. Probably something to do with wrapping.
    }

    //println!("node: {:?}  computed_main: {}  computed_cross: {}  main_sum: {}  cross_max: {}", node.key(), computed_main, computed_cross, main_sum, cross_max);

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

        _=> {}
    }

    // Propagate the computed size back up the tree
    Size { main: computed_main, cross: computed_cross }
}
