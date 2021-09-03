mod common;
use common::*;

fn main() {

    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Pixels(1000.0));
    state.set_height(root, Units::Pixels(600.0));


    let child1 = state.add(Some(root));
    state.set_width(child1, Units::Pixels(100.0));
    state.set_height(child1, Units::Auto);
    state.set_child_top(child1, Units::Pixels(50.0));
    state.set_child_bottom(child1, Units::Pixels(50.0));


    let child2 = state.add(Some(child1));
    state.set_width(child2, Units::Pixels(100.0));
    state.set_height(child2, Units::Pixels(100.0));

    let child3 = state.add(Some(child1));
    state.set_width(child3, Units::Pixels(100.0));
    state.set_height(child3, Units::Pixels(100.0));


    layout(&mut state.cache, &state.tree, &state.style);


    render(state, root);
    
}