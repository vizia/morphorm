mod common;
use common::*;

fn main() {

    let mut state = State::default();

    let root = state.add(None);
    state.set_width(root, Units::Pixels(1000.0));
    state.set_height(root, Units::Pixels(600.0));

    // Set the root to a grid layout type with 4 stretch rows and 3 stretch columns 
    state.set_layout_type(root, LayoutType::Grid);
    state.set_grid_rows(root, vec![Units::Stretch(1.0), Units::Pixels(200.0)]);
    state.set_grid_cols(root, vec![Units::Stretch(1.0), Units::Stretch(1.0)]);
    // state.set_row_between(root, Units::Stretch(1.0));
    // state.set_col_between(root, Units::Stretch(1.0));
    // state.set_child_left(root, Units::Stretch(1.0));
    // state.set_child_top(root, Units::Pixels(20.0));
    // state.set_child_right(root, Units::Stretch(2.0));
    // state.set_child_bottom(root, Units::Pixels(20.0));

    let child1 = state.add(Some(root));
    state.set_row(child1, 0, 1);
    state.set_col(child1, 0, 1);


    let child2 = state.add(Some(root));
    state.set_row(child2, 0, 1);
    state.set_col(child2, 1, 1);

    let child3 = state.add(Some(root));
    state.set_row(child3, 1, 1);
    state.set_col(child3, 0, 1);


    let child4 = state.add(Some(root));
    state.set_row(child4, 1, 1);
    state.set_col(child4, 1, 1);

    



    layout(&mut state.cache, &state.tree, &state.style);


    render(state, root);
    
}