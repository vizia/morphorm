mod common;
use common::*;

fn main() {

    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));

    // Set the root to a grid layout type with 4 stretch rows and 3 stretch columns 
    world.set_layout_type(root, LayoutType::Grid);
    world.set_grid_rows(root, vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)]);
    world.set_grid_cols(root, vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)]);

    let child1 = world.add(Some(root));
    world.set_row(child1, 0, 1);
    world.set_col(child1, 0, 2);


    let child2 = world.add(Some(root));
    world.set_row(child2, 0, 1);
    world.set_col(child2, 2, 1);

    let child3 = world.add(Some(root));
    world.set_row(child3, 1, 2);
    world.set_col(child3, 0, 1);


        world.set_layout_type(child3, LayoutType::Grid);
        world.set_grid_rows(child3, vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)]);
        world.set_grid_cols(child3, vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)]);

        let child31 = world.add(Some(child3));
        world.set_row(child31, 0, 1);
        world.set_col(child31, 0, 2);


        let child32 = world.add(Some(child3));
        world.set_row(child32, 0, 1);
        world.set_col(child32, 2, 1);

        let child33 = world.add(Some(child3));
        world.set_row(child33, 1, 2);
        world.set_col(child33, 0, 1);

        let child34 = world.add(Some(child3));
        world.set_row(child34, 1, 1);
        world.set_col(child34, 1, 2);

        let child35 = world.add(Some(child3));
        world.set_row(child35, 3, 1);
        world.set_col(child35, 0, 1);

        let child36 = world.add(Some(child3));
        world.set_row(child36, 2, 2);
        world.set_col(child36, 1, 1);

        let child37 = world.add(Some(child3));
        world.set_row(child37, 2, 2);
        world.set_col(child37, 2, 1);


    let child4 = world.add(Some(root));
    world.set_row(child4, 1, 1);
    world.set_col(child4, 1, 2);

    let child5 = world.add(Some(root));
    world.set_row(child5, 3, 1);
    world.set_col(child5, 0, 1);

    let child6 = world.add(Some(root));
    world.set_row(child6, 2, 2);
    world.set_col(child6, 1, 1);

    let child7 = world.add(Some(root));
    world.set_row(child7, 2, 2);
    world.set_col(child7, 2, 1);

        world.set_layout_type(child7, LayoutType::Grid);
        world.set_grid_rows(child7, vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)]);
        world.set_grid_cols(child7, vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)]);

        let child71 = world.add(Some(child7));
        world.set_row(child71, 0, 1);
        world.set_col(child71, 0, 2);


        let child72 = world.add(Some(child7));
        world.set_row(child72, 0, 1);
        world.set_col(child72, 2, 1);

        let child73 = world.add(Some(child7));
        world.set_row(child73, 1, 2);
        world.set_col(child73, 0, 1);

        let child74 = world.add(Some(child7));
        world.set_row(child74, 1, 1);
        world.set_col(child74, 1, 2);

        let child75 = world.add(Some(child7));
        world.set_row(child75, 3, 1);
        world.set_col(child75, 0, 1);

        let child76 = world.add(Some(child7));
        world.set_row(child76, 2, 2);
        world.set_col(child76, 1, 1);

        let child77 = world.add(Some(child7));
        world.set_row(child77, 2, 2);
        world.set_col(child77, 2, 1);



    // let child5 = world.add(Some(root));



    layout(&mut world.cache, &world.tree, &world.store);


    render(world, root);
    
}