mod common;

use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Auto);

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(100.0));
    // world.set_position_type(node2, PositionType::SelfDirected);

    let node3 = world.add(Some(node1));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(100.0));
    world.set_top(node3, Units::Pixels(50.0));
    world.set_left(node3, Units::Pixels(100.0));
    world.set_position_type(node3, PositionType::SelfDirected);
    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}
