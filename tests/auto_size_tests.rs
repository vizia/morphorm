

use morphorm::*;
use morphorm_ecs::*;

/// Test of auto width on the root node with no children
#[test]
fn root_node_auto_width_no_child() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Auto);

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_root_width = world.cache.width(root);

    assert_eq!(computed_root_width, 0.0);
}

/// Test of auto width on the root node with one child with pixel width
#[test]
fn root_node_auto_width_one_child_pixel_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(400.0));
    world.set_height(root, Units::Pixels(400.0));

    let child = world.add(Some(root));
    world.set_width(root, Units::Auto);

    let child2 = world.add(Some(child));
    world.set_width(root, Units::Pixels(200.0));

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_width1 = world.cache.width(child);
    let computed_width2 = world.cache.width(child2);

    assert_eq!(computed_width1, 200.0);
    assert_eq!(computed_width2, 200.0);
}

/// Test of auto width on the root node with one child with percentage width
#[test]
fn root_node_auto_width_one_child_percentage_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Auto);

    let child = world.add(Some(root));
    world.set_width(root, Units::Percentage(50.0));

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_root_width = world.cache.width(root);
    let computed_child_width = world.cache.width(child);

    assert_eq!(computed_root_width, 0.0);
    assert_eq!(computed_child_width, 0.0);
}

/// Test of auto width on the root node with one child with stretch width
#[test]
fn root_node_auto_width_one_child_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Auto);

    let child = world.add(Some(root));
    world.set_width(root, Units::Stretch(1.0));

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_root_width = world.cache.width(root);
    let computed_child_width = world.cache.width(child);

    assert_eq!(computed_root_width, 0.0);
    assert_eq!(computed_child_width, 0.0);
}

/// Test of auto width on the root node with one child with auto width
#[test]
fn root_node_auto_width_one_child_auto_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Auto);

    let child = world.add(Some(root));
    world.set_width(root, Units::Auto);

    layout(&mut world.cache, &world.tree, &world.store);

    let computed_root_width = world.cache.width(root);
    let computed_child_width = world.cache.width(child);

    assert_eq!(computed_root_width, 0.0);
    assert_eq!(computed_child_width, 0.0);
}

// Test of auto width on the root node with one child with auto width and one grandchild with pixel width
// #[test]
// fn root_node_auto_width_one_child_auto_width_one_grandchild_auto_width() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Auto);

//     let child = world.add(Some(root));
//     world.set_width(root, Units::Auto);

//     let grandchild = world.add(Some(child));
//     world.set_width(root, Units::Pixels(200.0));

//     layout(&mut world.cache, &world.tree, &world.store);

//     let computed_root_width = world.cache.width(root);
//     let computed_child_width = world.cache.width(child);
//     let computed_grandchild_width = world.cache.width(grandchild);

//     assert_eq!(computed_root_width, 200.0);
//     assert_eq!(computed_child_width, 200.0);
//     assert_eq!(computed_grandchild_width, 200.0);
// }


