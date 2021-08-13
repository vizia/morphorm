

use morphorm::*;
use morphorm_ecs::*;

/// Test of pixel width on the root node only
#[test]
fn root_node_pixel_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_width = world.cache.width(root);

    assert_eq!(computed_width, 1000.0);
}

/// Test of pixel height on the root node only
#[test]
fn root_node_pixel_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_height(root, Units::Pixels(600.0));

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_height = world.cache.height(root);

    assert_eq!(computed_height, 600.0);
}

/// Test of percentage width on the root node only
#[test]
fn root_node_percentage_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Percentage(50.0));

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_width = world.cache.width(root);

    assert_eq!(computed_width, 0.0);
}

/// Test of percentage height on the root node only
#[test]
fn root_node_percentage_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_height(root, Units::Percentage(50.0));

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_height = world.cache.height(root);

    assert_eq!(computed_height, 0.0);
}

/// Test of stretch width on the root node only
#[test]
fn root_node_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Stretch(1.0));

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_width = world.cache.width(root);

    assert_eq!(computed_width, 0.0);
}

/// Test of stretch height on the root node only
#[test]
fn root_node_stretch_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_height(root, Units::Stretch(1.0));

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_height = world.cache.height(root);

    assert_eq!(computed_height, 0.0);
}


