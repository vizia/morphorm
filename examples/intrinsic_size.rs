mod common;
use common::*;

fn main() {

    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let child1 = world.add(Some(root));
    world.set_width(child1, Units::Stretch(1.0));
    world.set_height(child1, Units::Stretch(1.0));
    world.set_intrinsic_width(child1, |height| height);


    layout(&mut world.cache, &world.tree, &world.store);


    render(world, root);
    
}