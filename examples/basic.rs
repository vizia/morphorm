mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_space(root, Units::Stretch(1.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_child_space(node, Units::Stretch(1.0));

    let child = world.add(Some(node));
    world.set_width(child, Units::Pixels(100.0));
    world.set_height(child, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    render(world, root);
}
