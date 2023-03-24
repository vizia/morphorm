mod common;
use common::*;

fn main() {
    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));

    // let node = world.add(Some(root));
    // world.set_width(node, Units::Pixels(200.0));
    // world.set_height(node, Units::Stretch(1.0));
    // world.set_left(node, Units::Stretch(1.0));
    // world.set_right(node, Units::Stretch(1.0));

    // // let node1 = world.add(Some(node));
    // // world.set_width(node1, Units::Auto);
    // // world.set_height(node1, Units::Stretch(1.0));
    // // world.set_layout_type(node1, LayoutType::Row);
    // // world.set_content_size(node1, |_, _, height| (height.unwrap(), height.unwrap()));

    // // let node2 = world.add(Some(node));
    // // world.set_width(node2, Units::Stretch(1.0));
    // // world.set_height(node2, Units::Auto);
    // // world.set_layout_type(node2, LayoutType::Row);
    // // world.set_content_size(node2, |_, width, _| (width.unwrap(), width.unwrap() * 3.0));

    // root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));

    // world.set_layout_type(root, LayoutType::Row);

    // let node = world.add(Some(root));
    // world.set_width(node, Units::Pixels(100.0));
    // world.set_height(node, Units::Pixels(400.0));
    // world.set_max_height(node, Units::Percentage(50.0));

    // root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));
    // // world.set_layout_type(root, LayoutType::Row);
    // // world.set_child_left(root, Units::Stretch(1.0));

    // let node = world.add(Some(root));
    // world.set_width(node, Units::Stretch(1.0));
    // world.set_height(node, Units::Pixels(200.0));
    // // world.set_min_height(node, Units::Pixels(700.0));
    // world.set_top(node, Units::Stretch(1.0));
    // world.set_bottom(node, Units::Stretch(1.0));
    // // world.set_left(node, Units::Stretch(1.0));
    // // world.set_right(node, Units::Stretch(1.0));
    // // world.set_layout_type(node, LayoutType::Column);

    // // let node1 = world.add(Some(node));
    // // world.set_width(node1, Units::Stretch(1.0));
    // // world.set_height(node1, Units::Pixels(100.0));
    // // world.set_left(node1, Units::Stretch(1.0));
    // // world.set_right(node1, Units::Stretch(1.0));
    // // world.set_min_width(node1, Units::Pixels(300.0));
    // // world.set_min_left(node1, Units::Pixels(100.0));
    // // world.set_min_right(node1, Units::Pixels(200.0));
    // // world.set_position_type(node1, PositionType::SelfDirected);

    // // let node2 = world.add(Some(node));
    // // world.set_width(node2, Units::Pixels(100.0));
    // // world.set_height(node2, Units::Stretch(1.0));
    // // world.set_left(node2, Units::Stretch(1.0));
    // // world.set_position_type(node2, PositionType::SelfDirected);

    // root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));

    // let node = world.add(Some(root));
    // world.set_height(node, Units::Pixels(200.0));
    // world.set_width(node, Units::Pixels(200.0));
    // world.set_layout_type(node, LayoutType::Column);

    // let node1 = world.add(Some(node));
    // world.set_height(node1, Units::Stretch(1.0));
    // world.set_width(node1, Units::Stretch(1.0));
    // world.set_left(node1, Units::Stretch(1.0));
    // world.set_right(node1, Units::Stretch(1.0));
    // world.set_top(node1, Units::Stretch(1.0));
    // world.set_bottom(node1, Units::Stretch(1.0));
    // world.set_min_width(node1, Units::Pixels(300.0));
    // world.set_layout_type(node1, LayoutType::Column);

    // root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));

    // let node = world.add(Some(root));
    // world.set_width(node, Units::Auto);
    // world.set_height(node, Units::Stretch(1.0));

    // let node1 = world.add(Some(node));
    // world.set_width(node1, Units::Stretch(1.0));
    // world.set_height(node1, Units::Stretch(1.0));
    // world.set_min_width(node1, Units::Pixels(300.0));

    // let node2 = world.add(Some(node));
    // world.set_width(node2, Units::Stretch(1.0));
    // world.set_height(node2, Units::Stretch(1.0));

    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(200.0));
    world.set_height(node, Units::Auto);

    let child = world.add(Some(node));
    world.set_width(child, Units::Auto);
    world.set_height(child, Units::Auto);
    world.set_min_width(child, Units::Stretch(1.0));
    world.set_content_size(child, |_, width, height| {
        println!("{:?}, {:?}", width, height);
        (0.0, 0.0)
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    render(world, root);
}
