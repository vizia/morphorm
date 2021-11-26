mod common;
use common::*;

fn main() {

    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(1000.0));
    world.set_cross(root, Units::Pixels(600.0));
    world.set_child_space(root, Units::Stretch(1.0));

    world.set_layout_type(root, LayoutType::Row);

    let child1 = world.add(Some(root));
    world.set_main(child1, Units::Stretch(1.0));
    world.set_cross(child1, Units::Pixels(100.0));

    let child2 = world.add(Some(root));
    world.set_main(child2, Units::Stretch(1.0));
    world.set_cross(child2, Units::Pixels(100.0));

    let child3 = world.add(Some(root));
    world.set_main(child3, Units::Stretch(1.0));
    world.set_cross(child3, Units::Pixels(100.0));



    layout(&mut world.cache, &world.tree, &world.store);


    render(world, root);
    
}