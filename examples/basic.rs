mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(600.0));
    world.set_height(node, Units::Pixels(400.0));
    world.set_layout_type(node, LayoutType::Row);

    let node1 = world.add(Some(node));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Stretch(1.0));
    world.set_layout_type(node1, LayoutType::Row);
    world.set_content_main(node1, |_, height| height);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Stretch(1.0));
    world.set_height(node2, Units::Auto);
    world.set_layout_type(node2, LayoutType::Row);
    world.set_content_cross(node2, |_, width| width * 3.0);

    layout(&root, None, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}
