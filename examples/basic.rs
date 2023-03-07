mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    
    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Stretch(1.0));
    world.set_layout_type(node, LayoutType::Column);

    let node1 = world.add(Some(node));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Stretch(1.0));
    world.set_content_size(node1, |width| width / 2.0);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Auto);
    world.set_height(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |width| width / 2.0);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}
