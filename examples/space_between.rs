mod common;
use common::*;

fn main() {

    let mut state = State::default();

    let root = state.add(None).unwrap();
    root.set_width(&mut state, Units::Pixels(1000.0));
    root.set_height(&mut state, Units::Pixels(600.0));

    root.set_layout_type(&mut state, LayoutType::Row);
    root.set_col_between(&mut state, Units::Pixels(30.0));

    // Center children by specifying stretch space on all sides
    // This could be abstratced into a single method for convenience
    root.set_child_left(&mut state, Units::Stretch(1.0));
    root.set_child_right(&mut state, Units::Stretch(1.0));
    root.set_child_top(&mut state, Units::Stretch(1.0));
    root.set_child_bottom(&mut state, Units::Stretch(1.0));

    let child1 = state.add(Some(root)).unwrap();
    child1.set_width(&mut state, Units::Pixels(100.0));
    child1.set_height(&mut state, Units::Pixels(100.0));

    let child2 = state.add(Some(root)).unwrap();
    child2.set_width(&mut state, Units::Pixels(100.0));
    child2.set_height(&mut state, Units::Pixels(100.0));

    let child3 = state.add(Some(root)).unwrap();
    child3.set_width(&mut state, Units::Pixels(100.0));
    child3.set_height(&mut state, Units::Pixels(100.0));

    let child4 = state.add(Some(root)).unwrap();
    child4.set_width(&mut state, Units::Pixels(100.0));
    child4.set_height(&mut state, Units::Pixels(100.0));

    let child5 = state.add(Some(root)).unwrap();
    child5.set_width(&mut state, Units::Pixels(100.0));
    child5.set_height(&mut state, Units::Pixels(100.0));



    layout(&mut state.cache, &state.tree, &state.style);


    render(state, root);
    
}