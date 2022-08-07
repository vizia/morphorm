use crate::{Node, Cache, LayoutType, Units};
use crate::Units::*;

#[derive(Debug, Copy, Clone)]
pub struct BoxConstraints {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

// Perform layout on a node
pub fn layout<'a, N, C>(
    node: &N,
    bc: &BoxConstraints,
    cache: &mut C,
    tree: &'a <N as Node<'a>>::Tree,
    store: &'a <N as Node<'a>>::Store,
) -> Size
where
    N: Node<'a>,
    C: Cache<Node = N::CacheKey>,
{

    let layout_type = node.layout_type(store).unwrap_or_default();

    let width = node.width(store).unwrap_or(Units::Stretch(1.0));
    let height = node.height(store).unwrap_or(Units::Stretch(1.0));

    let mut computed_width = 0.0;
    let mut computed_height = 0.0;

    match width {
        Pixels(width) => {
            computed_width = width;
        }

        Percentage(val) => {
            computed_width = (bc.max.0 * (val / 100.0)).round();
        }

        Stretch(_) => {
            computed_width = bc.max.0;
        }

        Auto => {
            match layout_type {
                LayoutType::Row => {

                    println!("E: {:?}  PBC: {:?}", node.key(), bc);

                    let cross_size = match height {
                        Units::Pixels(px) => Some(px),
                        Units::Percentage(pc) => Some((bc.max.1 * (pc / 100.0)).round()),
                        _=> Some(bc.min.0),
                    };

                    computed_width = if let Some(content_size) = cross_size.and_then(|cross_size| node.content_size(store, cross_size)) {
                        content_size
                    } else {
                        bc.min.1
                    };
                }

                _=> {}
            }
        }
    }

    match height {
        Pixels(height) => {
            computed_height = height;
        }

        Auto => {
            match layout_type {
                LayoutType::Row => {

                    // let cross_size = match height {
                    //     Units::Pixels(px) => Some(px),
                    //     Units::Percentage(pc) => Some((bc.max.1 * (pc / 100.0)).round()),
                    //     _=> Some(bc.min.0),
                    // };

                    computed_height = if let Some(content_size) = node.content_size(store, computed_width) {
                        content_size
                    } else {
                        bc.min.1
                    };
                }

                _=> {}
            }
        }

        _=> {}
    }

    //println!("Entity: {:?}  Computed Width: {}", node.key(), computed_width);


    let mut main_sum = 0.0;
    let mut cross_sum = 0.0;
    match layout_type {
        LayoutType::Row => {

            // Measure non-flex children on main axis
            let mut main_non_flex = 0.0;
            let mut flex_sum = 0.0;
            for child in node.children(tree) {

                let child_width = child.width(store).unwrap_or(Units::Stretch(1.0));

                match child_width {
                    Stretch(factor) => flex_sum += factor,

                    Pixels(width) => {

                        // Compute child box constraints
                        let child_bc = BoxConstraints {
                            min: (0.0, 0.0),
                            max: (width, std::f32::INFINITY),
                        };

                        let child_size = layout(&child, &child_bc, cache, tree, store, );
                        //println!("child size: {:?}", child_size);
                        main_sum += child_size.width;
                        main_non_flex += child_size.width;
                    }

                    Auto => {

                        // If the cross_size is definite and the content_size is some, then compute the main size from the content size
                        // if let Units::Pixels(cross_size) = child.height(store).unwrap_or(Units::Stretch(1.0)) {
                        //     if let Some(content_size) = child.content_size(store, cross_size) {
                        //         println!("is auto with definite height and content size: {}", content_size);
                        //         main_non_flex += content_size;
                        //     }
                        // }

                        let child_bc = BoxConstraints {
                            min: (0.0, 0.0),
                            max: (std::f32::INFINITY, std::f32::INFINITY),
                        };

                        let child_size = layout(&child, &child_bc, cache, tree, store, );
                        println!("child size: {:?}", child_size);
                        
                        main_non_flex += child_size.width;
                    }

                    _=> {}
                }

            }

            // Calculate free space
            let free_space = (bc.max.0 - main_non_flex).max(0.0);
            let mut remainder: f32 = 0.0;

            let mut major_flex: f32 = 0.0;
            let px_per_flex = free_space / flex_sum;

            // Compute flexible children
            for child in node.children(tree) {
                let child_width = child.width(store).unwrap_or(Units::Stretch(1.0));
                
                match child_width {
                    Stretch(factor) => {
                        let desired_major = factor * px_per_flex + remainder;
                        let actual_major = desired_major.round();
                        remainder = desired_major - actual_major;

                        let child_bc = BoxConstraints {
                            min: (actual_major, 0.0),
                            max: (actual_major, 0.0),
                        };

                        let child_size = layout(&child, &child_bc, cache, tree, store);

                        if child_size.width.is_finite() {
                            main_sum += child_size.width;
                        } else {
                            println!("WARNING: Flex child in Auto parent");
                        }

                    }

                    _=> {}
                }
            }

            // Position children
            let mut posx = 0.0;
            for child in node.children(tree) {
                let child_width = cache.width(child.key());
                cache.set_posx(child.key(), posx);
                posx += child_width;
            }


        }

        LayoutType::Column => {

        }

        _=> {}
    }

    //println!("main sum {:?} {}", node.key(), main_sum);

    match width {
        Auto => {
            if node.content_size(store, 0.0).is_none() {
                computed_width = main_sum;
            }
        }

        _=> {}
    }

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

    //println!("node: {:?}  bc: {:?}  computed_width: {}  computed_height: {}", node.key(), bc, computed_width, computed_height);

    cache.set_width(node.key(), computed_width);
    cache.set_height(node.key(), computed_height);


    Size {
        width: computed_width,
        height: computed_height,
    }
}

// Given a node and its parent box constraints, compute the main size of the node
fn compute_main_size<'a, N, C>(node: &N, bc: &BoxConstraints, cache: &mut C, store: &'a <N as Node<'a>>::Store) -> f32 
where
    N: Node<'a>,
    C: Cache<Node = N::CacheKey>,
{
    
    let width = node.width(store).unwrap_or(Units::Stretch(1.0));
    let height = node.height(store).unwrap_or(Units::Stretch(1.0));

    let (main_size, cross_size) = match node.layout_type(store).unwrap_or_default() {
        LayoutType::Row => {
            (width, height)
        }

        LayoutType::Column => {
            (height, width)
        }

        _=> unreachable!(),
    };

    //let width = node.width(store).unwrap_or(Units::Stretch(1.0));

    match main_size {
        Units::Pixels(px) => px,
        Units::Percentage(pc) => (bc.max.0 * (pc / 100.0)).round(),
        Units::Stretch(_) => bc.max.0,
        Units::Auto => {

            let cross_size = match cross_size {
                Units::Pixels(px) => Some(px),
                Units::Percentage(pc) => Some((bc.max.1 * (pc / 100.0)).round()),
                _=> None,
            };

            if let Some(content_size) = cross_size.and_then(|cross_size| node.content_size(store, cross_size)) {
                content_size
            } else {
                bc.min.1
            }
        }
    }
}