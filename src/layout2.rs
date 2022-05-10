
use crate::{Node, Cache, LayoutType, Units};
use crate::Units::*;
pub struct BoxConstraints {
    min: (f32, f32),
    max: (f32, f32),
}

pub struct Size {
    width: f32,
    height: f32,
}

// Perform layout on a node
pub fn layout<'a, N, C>(
    node: &N,
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
    let height = node.width(store).unwrap_or(Units::Stretch(1.0));

    let mut computed_width = 0.0;
    let mut computed_height = 0.0;

    if let Pixels(px) = width {
        computed_width = px;
    }

    if let Pixels(px) = height {
        computed_height = px;
    }

    match layout_type {
        LayoutType::Row => {
            let major = match width {
                Pixels(pixels) => {
                    pixels
                }

                Percentage(percentage) => {
                    percentage
                }

                Stretch(factor) => {
                    factor
                }

                Auto => {
                    0.0
                }
            };

            let mut major_non_flex = 0.0;
            let mut flex_sum = 0.0;
            // for child in node.children(tree) {
            //     let child_width = child.width(store).unwrap_or(Units::Stretch(1.0));
            //     let child_size = layout(&child, cache, tree, store);
            // }


        }

        LayoutType::Column => {

        }

        _=> {}
    }

    // Determine any fixed sizes
    // Compute the constraints
    // Pass the constraints to the children to compute their size
    // Compute fixed-size children
    // Compute stretch-size children
    for child in node.children(tree) {
        let width = child.width(store);

        let size = layout(&child, cache, tree, store);
    }

    cache.set_width(node.key(), computed_width);
    cache.set_height(node.key(), computed_height);

    Size {
        width: computed_width,
        height: computed_height,
    }
}