mod common;
use common::*;

fn main() {

    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Pixels(1000.0));
    state.set_height(root, Units::Pixels(600.0));


    let child1 = state.add(Some(root));
    state.set_width(child1, Units::Auto);
    state.set_min_width(child1, Units::Pixels(100.0));
    state.set_height(child1, Units::Pixels(200.0));
    state.set_left(child1, Units::Pixels(50.0));
    state.set_top(child1, Units::Pixels(50.0));


    layout(&mut state.cache, &state.tree, &state.style);


    render(state, root);
    
}