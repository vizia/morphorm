mod common;
use common::*;

fn main() {

    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Pixels(1000.0));
    state.set_height(root, Units::Pixels(600.0));

    state.set_layout_type(root, LayoutType::Row);
    state.set_col_between(root, Units::Pixels(30.0));

    // Center children by specifying stretch space on all sides
    // This could be abstratced into a single method for convenience
    state.set_child_left(root, Units::Stretch(1.0));
    state.set_child_right(root, Units::Stretch(1.0));
    state.set_child_top(root, Units::Stretch(1.0));
    state.set_child_bottom(root, Units::Stretch(1.0));

    let child1 = state.add(Some(root));
    state.set_width(child1, Units::Pixels(100.0));
    state.set_height(child1, Units::Pixels(100.0));

    let child2 = state.add(Some(root));
    state.set_width(child2, Units::Stretch(1.0));
    state.set_height(child2, Units::Pixels(100.0));

    let child3 = state.add(Some(root));
    state.set_width(child3, Units::Pixels(150.0));
    state.set_height(child3, Units::Pixels(100.0));

    let child4 = state.add(Some(root));
    state.set_width(child4, Units::Percentage(20.0));
    state.set_height(child4, Units::Pixels(100.0));

    let child5 = state.add(Some(root));
    state.set_width(child5, Units::Stretch(2.0));
    state.set_height(child5, Units::Pixels(100.0));



    layout(&mut state.cache, &state.tree, &state.style);




    render(state, root);
    
}