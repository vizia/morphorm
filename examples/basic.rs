mod common;
use std::alloc::Layout;

use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    // world.set_cross(node, Units::Percentage(25.0));
    world.set_main(node, Units::Stretch(1.0));
    // world.set_cross(node, Units::Pixels(200.0));
    world.set_cross(node, Units::Auto);
    world.set_content_size(node, |width| width);
    // world.set_cross_before(node, Units::Percentage(50.0));


    let node2 = world.add(Some(root));
    world.set_main(node2, Units::Stretch(1.0));


    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    // assert_eq!(
    //     world.cache.bounds(node),
    //     Some(&Rect { posx: 0.0, posy: 300.0, width: 150.0, height: 150.0 })
    // );

    // world.set_layout_type(root, LayoutType::Column);

    // let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    // layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    // assert_eq!(
    //     world.cache.bounds(node),
    //     Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 150.0 })
    // );

    render(world, root);
}
