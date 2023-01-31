mod common;

use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Column);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}
