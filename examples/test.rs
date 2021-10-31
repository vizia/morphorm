mod common;
use common::*;

fn main() {

    let mut world = World::default();

    let root = world.add(None);

    world.set_layout_type(root, LayoutType::Row);
    world.set_child_space(root, Units::Pixels(10.0));

    // let node0 = world.add(Some(root));
    // world.set_width(node0, Units::Stretch(1.0));
    // world.set_height(node0, Units::Stretch(1.0));
    // world.set_layout_type(node0, LayoutType::Row);
    // world.set_min_width(node0, Units::Pixels(0.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(100.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(100.0));
    // world.set_left(node3, Units::Pixels(50.0));

    let node4 = world.add(Some(root));
    world.set_width(node4, Units::Pixels(100.0));
    world.set_height(node4, Units::Pixels(100.0));

    // let node2 = world.add(Some(node));
    // world.set_width(node2, Units::Pixels(60.0));
    // world.set_height(node2, Units::Pixels(40.0));



    layout(&mut world.cache, &world.tree, &world.store);


    render(world, root);
    
}