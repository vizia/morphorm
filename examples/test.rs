mod common;
use std::alloc::Layout;

use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_col_between(node, Units::Stretch(1.0));
    world.set_child_space(node, Units::Stretch(1.0));
    world.set_layout_type(node, LayoutType::Row);

    let node0 = world.add(Some(node));
    world.set_width(node0, Units::Pixels(60.0));
    world.set_height(node0, Units::Pixels(60.0));

    let node1 = world.add(Some(node));
    world.set_width(node1, Units::Pixels(60.0));
    world.set_height(node1, Units::Pixels(60.0));
    world.set_max_left(node1, Units::Pixels(20.0));

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(60.0));
    world.set_height(node2, Units::Pixels(60.0));
    world.set_max_left(node2, Units::Pixels(20.0));

    layout(&mut world.cache, &world.tree, &world.store);

    render(world, root);
}
