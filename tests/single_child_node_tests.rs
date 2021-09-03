

use morphorm::*;
use morphorm_ecs::*;

/// Size Tests

/// Test of pixel width on a single child node of the root node 
#[test]
fn single_child_node_pixel_width() {
    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Pixels(1000.0));
    state.set_height(root, Units::Pixels(600.0));

    let child = state.add(Some(root));
    state.set_width(child, Units::Pixels(200.0));

    layout(&mut state.cache, &state.tree, &state.style);

    let computed_width = state.cache.width(child);

    assert_eq!(computed_width, 200.0);
}


/// Test of pixel height on a single child node of the root node 
#[test]
fn single_child_node_pixel_height() {
    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Pixels(1000.0));
    state.set_height(root, Units::Pixels(600.0));

    let child = state.add(Some(root));
    state.set_height(child, Units::Pixels(300.0));

    layout(&mut state.cache, &state.tree, &state.style);

    let computed_height = state.cache.height(child);

    assert_eq!(computed_height, 300.0);
}


/// Test of pixel height on a single child node of the root node 
#[test]
fn single_child_node_percentage_width() {
    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Pixels(1000.0));
    state.set_height(root, Units::Pixels(600.0));

    let child = state.add(Some(root));
    state.set_width(child, Units::Percentage(50.0));

    layout(&mut state.cache, &state.tree, &state.style);

    let computed_width = state.cache.width(child);

    assert_eq!(computed_width, 500.0);
}

