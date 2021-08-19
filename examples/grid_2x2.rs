mod common;
use common::*;

fn main() {

    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));

    // Set the root to a grid layout type with 4 stretch rows and 3 stretch columns 
    world.set_layout_type(root, LayoutType::Grid);
    world.set_grid_rows(root, vec![Units::Stretch(1.0), Units::Pixels(200.0)]);
    world.set_grid_cols(root, vec![Units::Stretch(1.0), Units::Stretch(1.0)]);
    // world.set_row_between(root, Units::Stretch(1.0));
    // world.set_col_between(root, Units::Stretch(1.0));
    // world.set_child_left(root, Units::Stretch(1.0));
    // world.set_child_top(root, Units::Pixels(20.0));
    // world.set_child_right(root, Units::Stretch(2.0));
    // world.set_child_bottom(root, Units::Pixels(20.0));

    let child1 = world.add(Some(root));
    world.set_row(child1, 0, 1);
    world.set_col(child1, 0, 1);


    let child2 = world.add(Some(root));
    world.set_row(child2, 0, 1);
    world.set_col(child2, 1, 1);

    let child3 = world.add(Some(root));
    world.set_row(child3, 1, 1);
    world.set_col(child3, 0, 1);


    let child4 = world.add(Some(root));
    world.set_row(child4, 1, 1);
    world.set_col(child4, 1, 1);

    



    layout(&mut world.cache, &world.tree, &world.store);


    render(world, root);
    
}