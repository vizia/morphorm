mod common;
use common::*;

fn main() {

    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let child1 = world.add(Some(root));
    world.set_width(child1, Units::Pixels(100.0));
    world.set_height(child1, Units::Pixels(100.0));

    let child2 = world.add(Some(root));
    world.set_width(child2, Units::Pixels(100.0));
    world.set_height(child2, Units::Pixels(100.0));

    let child3 = world.add(Some(root));
    world.set_width(child3, Units::Pixels(100.0));
    world.set_height(child3, Units::Pixels(100.0));



    layout(&mut world.cache, &world.tree, &world.store);


    render(world, root);
    
}