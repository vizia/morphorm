mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopCenter);
    // world.set_padding(root, Units::Pixels(50.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_horizontal_gap(root, Units::Stretch(1.0));
    world.set_min_horizontal_gap(root, Units::Pixels(50.0));
    world.set_max_horizontal_gap(root, Units::Pixels(300.0));
    // world.set_vertical_gap(root, Units::Stretch(1.0));
    // world.set_min_vertical_gap(root, Units::Pixels(100.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Pixels(100.0));
    world.set_layout_type(node, LayoutType::Row);
    world.set_absolute_auto(node, true);
    // world.set_min_width(node, Units::Pixels(100.0));

    let node = world.add(Some(node));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_layout_type(node, LayoutType::Row);
    world.set_position_type(node, PositionType::Absolute);
    // world.set_max_width(node, Units::Pixels(200.0));

    // let node = world.add(Some(root));
    // world.set_width(node, Units::Pixels(100.0));
    // world.set_height(node, Units::Pixels(100.0));
    // world.set_layout_type(node, LayoutType::Row);

    // let child = world.add(Some(node));
    // world.set_width(child, Units::Stretch(1.0));
    // world.set_height(child, Units::Auto);
    // world.set_layout_type(child, LayoutType::Row);
    // world.set_max_width(child, Units::Auto);

    // let subchild = world.add(Some(child));
    // world.set_width(subchild, Units::Pixels(100.0));
    // world.set_height(subchild, Units::Pixels(50.0));

    // let subchild = world.add(Some(child));
    // world.set_width(subchild, Units::Pixels(100.0));
    // world.set_height(subchild, Units::Pixels(50.0));

    // let subchild = world.add(Some(child));
    // world.set_width(subchild, Units::Pixels(100.0));
    // world.set_height(subchild, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    render(world, root);
}
