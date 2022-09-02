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


    let mut main_non_flex = 0.0;
    let mut main_flex_sum = 0.0;
    let mut flex_line = Vec::new();

    // Compute non-flexible children
    for child in node.children(tree) {
        let child_main = child.main(store).unwrap_or(Units::Stretch(1.0));

        match child_main {
            Stretch(factor) => {
                main_flex_sum += factor;
                flex_line.push(child);
            },

            Pixels(val) => {
                // Compute child box constraints
                let child_bc = BoxConstraints {
                    min: (0.0, 0.0),
                    max: (val, computed_cross),
                };

                let child_size = layout(&child, layout_type, &child_bc, cache, tree, store);

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

                let child_bc = BoxConstraints {
                    min: (0.0, 0.0),
                    max: (computed_main, computed_cross),
                };

                let child_size = layout(&child, layout_type, &child_bc, cache, tree, store);

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
                    BoxConstraints { min: (actual_main, computed_cross), max: (actual_main, computed_cross) };

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
    }

    // Position children
    let parent_posx = cache.posx(node.key());
    let parent_posy = cache.posx(node.key());

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
                    cache.set_posy(child.key(), parent_posy + main_pos);
                    cache.set_posx(child.key(), parent_posx + cross_pos);
                    let child_height = cache.height(child.key());
                    main_pos += child_height;
                }

                _=> {}
            }
        }
        // TODO - add the height of the flex line
        cross_pos += 0.0;
        main_pos = 0.0; // I can't remember why this is here or what it does. Probably something to do with wrapping.
    }

    //println!("node: {:?}  computed_main: {}  computed_cross: {}  main_sum: {}  cross_max: {}", node.key(), computed_main, computed_cross, main_sum, cross_max);

    // Constrain the computed size to the sum/max of the children.
    // This is also how content-size gets propagated up the tree.
    // TODO: Constrain to min/max size when added
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
