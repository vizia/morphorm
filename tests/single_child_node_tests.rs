

use morphorm::*;
use morphorm_ecs::*;

/// Size Tests

/// Test of pixel width on a single child node of the root node 
#[test]
fn single_child_node_pixel_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));

    let child = world.add(Some(root));
    world.set_width(child, Units::Pixels(200.0));

    layout(&mut world.node_cache, &world.visual_tree, &world.components);

    let computed_width = world.node_cache.width(&child);

    assert_eq!(computed_width, 200.0);
}


/// Test of pixel height on a single child node of the root node 
#[test]
fn single_child_node_pixel_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));

    let child = world.add(Some(root));
    world.set_height(child, Units::Pixels(300.0));

    layout(&mut world.node_cache, &world.visual_tree, &world.components);

    let computed_height = world.node_cache.height(&child);

    assert_eq!(computed_height, 300.0);
}


/// Test of pixel height on a single child node of the root node 
#[test]
fn single_child_node_percentage_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));

    let child = world.add(Some(root));
    world.set_width(child, Units::Percentage(50.0));

    layout(&mut world.node_cache, &world.visual_tree, &world.components);

    let computed_width = world.node_cache.width(&child);

    assert_eq!(computed_width, 500.0);
}

