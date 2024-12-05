mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    // world.set_layout_type(root, LayoutType::Row);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(100.0));
    world.set_alignment(root, Alignment::Center);
    world.set_vertical_scroll(root, 1.0);
    world.set_horizontal_scroll(root, 0.0);
    

    let card = world.add(Some(root));
    // world.set_position_type(card, PositionType::SelfDirected);
    // world.set_layout_type(card, LayoutType::Row);
    
    // world.set_left(card, Units::Stretch(0.3));
    // world.set_right(card, Units::Stretch(0.7));
    // world.set_top(card, Units::Stretch(1.0));
    // world.set_bottom(card, Units::Stretch(1.0));
    world.set_width(card, Units::Pixels(800.0));
    world.set_height(card, Units::Percentage(1000.0));
    
    // world.set_padding_left(card, Units::Pixels(10.0));
    // world.set_padding_right(card, Units::Pixels(10.0));
    world.set_alignment(card, Alignment::Center);

    // let node1 = world.add(Some(card));
    // // world.set_layout_type(node1, LayoutType::Row);
    // world.set_width(node1, Units::Stretch(1.0));
    // world.set_height(node1, Units::Pixels(10.0));
    // world.set_alignment(node1, Alignment::Center);
    // // world.set_horizontal_gap(node1, Units::Pixels(20.0));

    // let node2 = world.add(Some(card));
    // // world.set_layout_type(node, LayoutType::Row);
    // world.set_width(node2, Units::Pixels(100.0));
    // world.set_height(node2, Units::Pixels(50.0));

    // let node2 = world.add(Some(node1));
    // // world.set_layout_type(node, LayoutType::Row);
    // world.set_width(node2, Units::Pixels(100.0));
    // world.set_height(node2, Units::Stretch(1.0));


    // let node3 = world.add(Some(node2));
    // world.set_layout_type(node3, LayoutType::Row);
    // world.set_width(node3, Units::Stretch(1.0));
    // world.set_height(node3, Units::Auto);

    // let child = world.add(Some(node3));
    // world.set_width(child, Units::Stretch(1.0));
    // world.set_height(child, Units::Pixels(20.0));

    // let child = world.add(Some(node3));
    // world.set_width(child, Units::Pixels(20.0));
    // world.set_height(child, Units::Pixels(20.0));

    // let node3 = world.add(Some(node2));
    // world.set_layout_type(node3, LayoutType::Row);
    // world.set_width(node3, Units::Stretch(1.0));
    // world.set_height(node3, Units::Auto);

    // let child = world.add(Some(node3));
    // world.set_width(child, Units::Stretch(1.0));
    // world.set_height(child, Units::Pixels(20.0));

    // let child = world.add(Some(node3));
    // world.set_width(child, Units::Pixels(20.0));
    // world.set_height(child, Units::Pixels(20.0));

    // let node3 = world.add(Some(node2));
    // world.set_layout_type(node3, LayoutType::Row);
    // world.set_width(node3, Units::Stretch(1.0));
    // world.set_height(node3, Units::Auto);

    // let child = world.add(Some(node3));
    // world.set_width(child, Units::Stretch(1.0));
    // world.set_height(child, Units::Pixels(20.0));

    // let child = world.add(Some(node3));
    // world.set_width(child, Units::Pixels(20.0));
    // world.set_height(child, Units::Pixels(20.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    render(world, root);
}
