mod common;
use common::*;

fn main() {


    let test1 = Units2::pixels(100.0);
    println!("Test1: {:?}", test1);

    let test2 = Units2::pixels(100.0).min(50.0);
    println!("Test2: {:?}", test2);

    let test3 = Units2::pixels(100.0).min(50.0).max(150.0);
    println!("Test3: {:?}", test3);

    // let mut state = State::default();

    // let root = state.add(None);
    // state.set_width(root, Units::Pixels(1000.0));
    // state.set_height(root, Units::Pixels(600.0));
    // state.set_left(root, Units::Pixels(0.0));
    // state.set_top(root, Units::Pixels(0.0));

    // let child = state.add(Some(root));
    // state.set_width(child, Units::Percentage(50.0));
    // state.set_height(child, Units::Stretch(1.0));

    // // let _child2 = state.add(Some(root));
    // // state.set_width(child, Units::Stretch(1.0));
    // // state.set_height(child, Units::Stretch(1.0));
    // // state.set_left(child, Units::Pixels(50.0));


    // layout(&mut state.cache, &state.tree, &state.style);

    // let computed_width = state.cache.width(&child);
    // let computed_height = state.cache.height(&child);
    // let computed_posx = state.cache.posx(&child);
    // let computed_posy = state.cache.posy(&child);

    // println!("Computed Width: {}", computed_width);
    // println!("Computed Height: {}", computed_height);
    // println!("Computed PosX: {}", computed_posx);
    // println!("Computed PosY: {}", computed_posy);

    // render(state, root);
    
}
