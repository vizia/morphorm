mod common;
use common::*;

fn main() {

    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Pixels(1000.0));
    state.set_height(root, Units::Pixels(600.0));

    // Center children (horizontally) by specifying stretch space on the left and right
    state.set_child_left(root, Units::Stretch(1.0));
    state.set_child_right(root, Units::Stretch(1.0));

    let child1 = state.add(Some(root));
    state.set_width(child1, Units::Pixels(100.0));
    state.set_height(child1, Units::Pixels(100.0));

    let child2 = state.add(Some(root));
    state.set_width(child2, Units::Pixels(100.0));
    state.set_height(child2, Units::Pixels(100.0));


    layout(&mut state.cache, &state.tree, &state.style);


    render(state, root);
    
}