mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));

    let child1 = world.add(Some(root));
    world.set_width(child1, Units::Auto);
    world.set_min_width(child1, Units::Pixels(100.0));
    world.set_height(child1, Units::Pixels(200.0));
    world.set_left(child1, Units::Pixels(50.0));
    world.set_top(child1, Units::Pixels(50.0));

    layout(&mut world.cache, &world.tree, &world.store);

    render(world, root);
}
