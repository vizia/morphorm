mod common;

use common::*;

fn build_shallow_tree(world: &mut World, parent: Option<Entity>, depth: usize) {
    if depth > 0 {
        let node = world.add(parent);
        world.set_all_stretch(node);
        for _ in 0..10 {
            build_shallow_tree(world, Some(node), depth - 1)
        }
    }
}

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(200.0));
    world.set_layout_type(node1, LayoutType::Row);

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Auto);
    world.set_height(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |height| height);


    // build_shallow_tree(&mut world, Some(root), 3);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}
