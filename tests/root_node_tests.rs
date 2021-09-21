

use morphorm::*;
use morphorm_ecs::*;

/// Test of pixel width on the root node only
#[test]
fn node_pixel_width() {
    let mut world = World::default();

    let root = world.add(None);

    let child = world.add(Some(root));
    world.set_width(child, Units::Pixels(1000.0));

    layout(&mut state.cache, &state.tree, &state.style);

    let computed_width = world.cache.width(child);

    assert_eq!(computed_width, 1000.0);
}

/// Test of pixel height on the root node only
#[test]
fn node_pixel_height() {
    let mut world = World::default();

    let root = world.add(None);

    let child = world.add(Some(root));
    world.set_height(child, Units::Pixels(1000.0));

    layout(&mut state.cache, &state.tree, &state.style);

    let computed_height = world.cache.height(child);

    assert_eq!(computed_height, 1000.0);
}

/// Test of percentage width on the root node only
#[test]
fn root_node_percentage_width() {
    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Percentage(50.0));

    layout(&mut state.cache, &state.tree, &state.style);

    let computed_width = state.cache.width(root);

    assert_eq!(computed_width, 0.0);
}

/// Test of percentage height on the root node only
#[test]
fn root_node_percentage_height() {
    let mut state = State::default();

    let root = state.add(None);
    state.set_height(root, Units::Percentage(50.0));

    layout(&mut state.cache, &state.tree, &state.style);

    let computed_height = state.cache.height(root);

    assert_eq!(computed_height, 0.0);
}

/// Test of stretch width on the root node only
#[test]
fn root_node_stretch_width() {
    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Stretch(1.0));

    layout(&mut state.cache, &state.tree, &state.style);

    let computed_width = state.cache.width(root);

    assert_eq!(computed_width, 0.0);
}

/// Test of stretch height on the root node only
#[test]
fn root_node_stretch_height() {
    let mut state = State::default();

    let root = state.add(None);
    state.set_height(root, Units::Stretch(1.0));

    layout(&mut state.cache, &state.tree, &state.style);

    let computed_height = state.cache.height(root);

    assert_eq!(computed_height, 0.0);
}


