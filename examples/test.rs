mod common;
use common::*;

fn main() {

    let mut world = World::default();

    let root = world.add(None);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(110.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_child_space(node, Units::Stretch(1.0));

    let node0 = world.add(Some(node));
    world.set_width(node0, Units::Pixels(60.0));
    world.set_height(node0, Units::Pixels(40.0));

    let node1 = world.add(Some(node));
    world.set_width(node1, Units::Pixels(60.0));
    world.set_height(node1, Units::Pixels(40.0));
    world.set_position_type(node1, PositionType::SelfDirected);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(60.0));
    world.set_height(node2, Units::Pixels(40.0));



    layout(&mut world.cache, &world.tree, &world.store);


    render(world, root);
    
}