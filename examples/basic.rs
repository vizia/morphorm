mod common;
use std::alloc::Layout;

use common::*;

fn main() {
    let mut world = World::default();

    // let root = world.add(None);
    // world.set_main(root, Units::Pixels(600.0));
    // world.set_cross(root, Units::Pixels(600.0));

    // let node1 = world.add(Some(root));
    // world.set_main(node1, Units::Auto);
    // world.set_cross(node1, Units::Pixels(150.0));

    // let node2 = world.add(Some(node1));
    // world.set_main(node2, Units::Pixels(400.0));
    // world.set_cross(node2, Units::Pixels(150.0));

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Column);
    world.set_child_main_before(root, Units::Pixels(50.0));
    world.set_child_main_after(root, Units::Pixels(50.0));
    world.set_child_cross_before(root, Units::Pixels(50.0));
    world.set_child_cross_after(root, Units::Pixels(50.0));

    let a = world.add(Some(root));
    world.set_main(a, Units::Auto);
    world.set_cross(a, Units::Stretch(1.0));
    world.set_child_space(a, Units::Pixels(30.0));
    // world.set_content_size(a, |main| main);
    world.set_layout_type(a, LayoutType::Column);
    // world.set_position_type(a, PositionType::SelfDirected);
    // world.set_cross_before(a, Units::Stretch(1.0));
    // world.set_cross_after(a, Units::Stretch(1.0));

    let node1 = world.add(Some(root));
    world.set_main(node1, Units::Stretch(1.0));
    world.set_cross(node1, Units::Stretch(1.0));
    // // world.set_position_type(node1, PositionType::SelfDirected);
    // world.set_content_size(node1, |main| main);
    // // world.set_main_before(node1, Units::Stretch(1.0));
    // world.set_layout_type(node1, LayoutType::Column);

    let node2 = world.add(Some(a));
    world.set_main(node2, Units::Auto);
    world.set_cross(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |main| main);
    // world.set_position_type(node2, PositionType::SelfDirected);
    // world.set_cross_before(node2, Units::Stretch(1.0));
    // world.set_cross_after(node2, Units::Stretch(1.0));

    // let node3 = world.add(Some(a));
    // world.set_main(node3, Units::Stretch(1.0));
    // world.set_cross(node3, Units::Stretch(1.0));
    // world.set_position_type(node3, PositionType::SelfDirected);
    // world.set_cross_before(node3, Units::Stretch(1.0));

    // let node2 = world.add(Some(node1));
    // world.set_main(node2, Units::Pixels(300.0));
    // world.set_cross(node2, Units::Pixels(50.0));

    // let node3 = world.add(Some(root));
    // world.set_main(node3, Units::Stretch(1.0));
    // world.set_cross(node3, Units::Stretch(1.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}
