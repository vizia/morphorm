mod common;
use std::alloc::Layout;

use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_main(node1, Units::Pixels(400.0));
    world.set_cross(node1, Units::Stretch(1.0));
    // world.set_main_after(node1, Units::Pixels(50.0));
    world.set_cross_before(node1, Units::Pixels(50.0));
    world.set_cross_after(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_main(node2, Units::Pixels(400.0));
    world.set_cross(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}
