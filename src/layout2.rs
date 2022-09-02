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
    //println!("layout: {:?} bc: {:?}", node.key(), bc);
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

        _=> {}
    }

    // Compute fixed-size cross size
    match cross {
        Pixels(val) => {
            computed_cross = val;
        }

        Stretch(_) => {
            if bc.max.1.is_finite() {
                computed_cross = bc.max.1;
            }
        }

        _ => {}
    }

    // Apply content-size
    match layout_type {
        LayoutType::Row => {
            if parent_layout_type == LayoutType::Column && main == Units::Auto {
                if let Some(content_size) = node.content_size(store, computed_cross) {
                    computed_main = content_size
                }
            }

            if parent_layout_type == LayoutType::Row && cross == Units::Auto {
                if let Some(content_size) = node.content_size(store, computed_main) {
                    computed_cross = content_size
                }
            }
        }

        LayoutType::Column => {
            if parent_layout_type == LayoutType::Row && main == Units::Auto {
                if let Some(content_size) = node.content_size(store, computed_cross) {
                    computed_main = content_size;
                }
            }

            if parent_layout_type == LayoutType::Column && cross == Units::Auto {
                if let Some(content_size) = node.content_size(store, computed_main) {
                    computed_cross = content_size;
                }
            }
        }

        _=> {}
    }

    //println!("Entity: {:?}  Computed Width: {}", node.key(), computed_width);
    let mut flex_lines = Vec::new();

    let mut main_sum = 0.0f32;
    let mut cross_max = 0.0f32;

    //match layout_type {
        //LayoutType::Row => {
            // Measure non-flex children on main axis
            let mut main_non_flex = 0.0;
            let mut main_flex_sum = 0.0;
            let mut flex_line = Vec::new();

            for child in node.children(tree) {
                let child_main = child.main(store).unwrap_or(Units::Stretch(1.0));
                let child_cross = child.cross(store).unwrap_or(Units::Stretch(1.0));

                match child_main {
                    Stretch(factor) => {
                        main_flex_sum += factor;
                        flex_line.push(child);
                    },

                    Pixels(val) => {
                        // Compute child box constraints
                        let mut child_bc = BoxConstraints {
                            min: (0.0, 0.0),
                            max: (val, computed_cross),
                        };

                        // match child_cross {
                        //     Pixels(cross) => {
                        //         child_bc.max.1 = cross;
                        //     }

                        //     _=> {}
                        // }

                        //println!("Layout child: {:?} with {:?}", child.key(), child_bc);

                        let child_size = layout(&child, layout_type, &child_bc, cache, tree, store);
                        //println!("child size: {:?}", child_size);

                        // If the main_sum exceeds the parent width then add the node to the next flex line
                        if main_non_flex + child_size.main > computed_main {
                            //println!("push line: {:?}", child.key());
                            flex_lines.push((flex_line.clone(), main_non_flex));
                            flex_line.clear();
                            main_non_flex = 0.0;
                        }

                        main_sum += child_size.main;
                        cross_max = cross_max.max(child_size.cross);
                        main_non_flex += child_size.main;

                        flex_line.push(child);
                    }

                    Auto => {
                        // If the cross_size is definite and the content_size is some, then compute the main size from the content size
                        // if let Units::Pixels(cross_size) = child.height(store).unwrap_or(Units::Stretch(1.0)) {
                        //     if let Some(content_size) = child.content_size(store, cross_size) {
                        //         println!("is auto with definite height and content size: {}", content_size);
                        //         main_non_flex += content_size;
                        //     }
                        // }

                        let mut child_bc = BoxConstraints {
                            min: (0.0, 0.0),
                            max: (computed_main, computed_cross),
                        };

                        // match child_cross {
                        //     Pixels(cross) => {
                        //         child_bc.max.1 = cross;
                        //     }

                        //     _=> {}
                        // }

                        let child_size = layout(&child, layout_type, &child_bc, cache, tree, store);
                        //println!("child size: {:?}", child_size);

                        main_non_flex += child_size.main;

                        main_sum += child_size.main;
                        cross_max = cross_max.max(child_size.cross);

                        flex_line.push(child);
                    }

                    _ => {}
                }
            }

            flex_lines.push((flex_line.clone(), main_non_flex));

            // Calculate free space
            let free_main_space = (bc.max.0 - main_non_flex).max(0.0);
            let mut remainder: f32 = 0.0;

            let mut main_flex: f32 = 0.0;
            let main_px_per_flex = free_main_space / main_flex_sum;

            // Compute flexible children
            for child in node.children(tree) {
                let child_main = child.main(store).unwrap_or(Units::Stretch(1.0));

                match child_main {
                    Stretch(factor) => {
                        let desired_main = factor * main_px_per_flex + remainder;
                        let actual_main = desired_main.round();
                        remainder = desired_main - actual_main;

                        let child_bc =
                            BoxConstraints { min: (actual_main, 0.0), max: (actual_main, 0.0) };

                        let child_size = layout(&child, layout_type, &child_bc, cache, tree, store);

                        if child_size.main.is_finite() {
                            main_sum += child_size.main;
                            cross_max = cross_max.max(child_size.cross);
                        } else {
                            println!("WARNING: Flex child in Auto parent");
                        }
                    }

                    _ => {}
                }

                // match child_height {
                //     Stretch(factor) => {

                //     }
                // }
            }

            // Position children
            // let mut posx = 0.0;
            // for child in node.children(tree) {
            //     let child_width = cache.width(child.key());
            //     cache.set_posx(child.key(), posx);
            //     posx += child_width;
            // }

            let parent_posx = cache.posx(node.key());
            let parent_posy = cache.posx(node.key());

            // println!("{:?} {} {}", node.key(), parent_posx, parent_posy);


            let mut cross_pos = 0.0;
            for (line, size) in flex_lines.iter() {
                let mut main_pos = 0.0;
                for child in line.iter() {
                    match layout_type {
                        LayoutType::Row => {
                            cache.set_posx(child.key(), parent_posx + main_pos);
                            cache.set_posy(child.key(), parent_posy + cross_pos);
                            let child_width = cache.width(child.key());
                            main_pos += child_width;
                        }

                        LayoutType::Column => {
                            //println!("position: {:?}", child.key());
                            cache.set_posy(child.key(), parent_posy + main_pos);
                            cache.set_posx(child.key(), parent_posx + cross_pos);
                            let child_height = cache.height(child.key());
                            main_pos += child_height;
                        }

                        _=> {}
                    }
                }
                //let child_height = cache.height(child.key());
                cross_pos += 0.0;
                main_pos = 0.0;
            }
        //}

        //LayoutType::Column => {}

        //_ => {}
    //}

    //println!("main sum {:?} {}", node.key(), main_sum);

    // match main {
    //     Auto => {
    //         if node.content_size(store, 0.0).is_none() {
    //             computed_main = main_sum;
    //         }
    //     }

    //     _ => {}
    // }

    // Determine any fixed sizes
    // Compute the constraints
    // Pass the constraints to the children to compute their size
    // Compute fixed-size children
    // Compute stretch-size children
    // for child in node.children(tree) {
    //     let width = child.width(store).unwrap_or_default();
    //     let height = child.width(store).unwrap_or_default();

    //     if let Pixels(_) = width {
    //         // Compute size of child
    //         let size = layout(&child, cache, tree, store);
    //     }
    // }

    //println!("node: {:?}  computed_main: {}  computed_cross: {}  main_sum: {}  cross_max: {}", node.key(), computed_main, computed_cross, main_sum, cross_max);

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

    Size { main: computed_main, cross: computed_cross }
}

// Given a node and its parent box constraints, compute the main size of the node
// fn compute_main_size<'a, N, C>(
//     node: &N,
//     bc: &BoxConstraints,
//     cache: &mut C,
//     store: &'a <N as Node<'a>>::Store,
// ) -> f32
// where
//     N: Node<'a>,
//     C: Cache<Node = N::CacheKey>,
// {
//     let width = node.width(store).unwrap_or(Units::Stretch(1.0));
//     let height = node.height(store).unwrap_or(Units::Stretch(1.0));

//     let (main_size, cross_size) = match node.layout_type(store).unwrap_or_default() {
//         LayoutType::Row => (width, height),

//         LayoutType::Column => (height, width),

//         _ => unreachable!(),
//     };

//     //let width = node.width(store).unwrap_or(Units::Stretch(1.0));

//     match main_size {
//         Units::Pixels(px) => px,
//         Units::Percentage(pc) => (bc.max.0 * (pc / 100.0)).round(),
//         Units::Stretch(_) => bc.max.0,
//         Units::Auto => {
//             let cross_size = match cross_size {
//                 Units::Pixels(px) => Some(px),
//                 Units::Percentage(pc) => Some((bc.max.1 * (pc / 100.0)).round()),
//                 _ => None,
//             };

//             if let Some(content_size) =
//                 cross_size.and_then(|cross_size| node.content_size(store, cross_size))
//             {
//                 content_size
//             } else {
//                 bc.min.1
//             }
//         }
//     }
// }
