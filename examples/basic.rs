mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(100.0));
    world.set_height(root, Units::Pixels(100.0));
    //world.set_child_space(root, Units::Stretch(1.0));

    // world.set_layout_type(root, LayoutType::Row);

    let node0 = world.add(Some(root));
    world.set_width(node0, Units::Auto);
    world.set_height(node0, Units::Pixels(100.0));

    let node1 = world.add(Some(node0));
    world.set_width(node1, Units::Pixels(20.0));
    world.set_height(node1, Units::Pixels(40.0));


    let node2 = world.add(Some(node0));
    world.set_width(node2, Units::Pixels(50.0));
    world.set_height(node2, Units::Pixels(40.0));
    // world.set_child_space(parent, Units::Pixels(10.0));

    // world.set_layout_type(parent, LayoutType::Row);

    // let child1 = world.add(Some(parent));
    // world.set_width(child1, Units::Stretch(1.0));
    // world.set_height(child1, Units::Pixels(100.0));
    // world.set_min_width(child1, Units::Pixels(100.0));

    // let child2 = world.add(Some(parent));
    // world.set_width(child2, Units::Stretch(1.0));
    // world.set_height(child2, Units::Pixels(100.0));
    // world.set_min_width(child2, Units::Pixels(50.0));

    // let child3 = world.add(Some(parent));
    // world.set_width(child3, Units::Stretch(1.0));
    // world.set_height(child3, Units::Pixels(100.0));

    let root_bc = BoxConstraints {
        min: (100.0, 100.0),
        max: (100.0, 100.0),
    };

    layout(&root, &root_bc, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}
