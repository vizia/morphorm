mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::Center);
    // world.set_padding(root, Units::Pixels(50.0));
    // world.set_layout_type(root, LayoutType::Row);
    // world.set_horizontal_gap(root, Units::Stretch(1.0));
    // world.set_min_horizontal_gap(root, Units::Pixels(50.0));
    // world.set_max_horizontal_gap(root, Units::Pixels(300.0));
    // world.set_vertical_gap(root, Units::Stretch(1.0));
    // world.set_min_vertical_gap(root, Units::Pixels(100.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_padding(node, Units::Pixels(20.0));
    world.set_layout_type(node, LayoutType::Grid);
    world.set_grid_columns(node, vec![Units::Stretch(1.0), Units::Pixels(300.0)]);
    world.set_grid_rows(node, vec![Units::Pixels(100.0), Units::Stretch(1.0)]);
    world.set_horizontal_gap(node, Units::Pixels(20.0));
    world.set_vertical_gap(node, Units::Pixels(10.0));

    let child = world.add(Some(node));
    world.set_column_start(child, 0);
    world.set_column_span(child, 2);
    world.set_row_start(child, 0);

    let child = world.add(Some(node));
    world.set_column_start(child, 0);
    world.set_row_start(child, 1);
    world.set_padding(child, Units::Pixels(20.0));
    world.set_vertical_gap(child, Units::Pixels(20.0));

    let subchild = world.add(Some(child));
    let subchild = world.add(Some(child));
    let subchild = world.add(Some(child));

    let child = world.add(Some(node));
    world.set_column_start(child, 1);
    world.set_row_start(child, 1);
    world.set_layout_type(child, LayoutType::Grid);
    world.set_padding(child, Units::Pixels(20.0));
    world.set_alignment(child, Alignment::Center);
    world.set_grid_columns(child, vec![Units::Pixels(50.0), Units::Pixels(50.0)]);
    world.set_grid_rows(child, vec![Units::Pixels(50.0), Units::Pixels(50.0)]);
    world.set_horizontal_gap(child, Units::Pixels(10.0));
    world.set_vertical_gap(child, Units::Pixels(10.0));

    let subchild = world.add(Some(child));
    world.set_column_start(subchild, 0);
    world.set_row_start(subchild, 0);

    let subchild = world.add(Some(child));
    world.set_column_start(subchild, 1);
    world.set_row_start(subchild, 0);
    world.set_row_span(subchild, 2);

    let subsubchild = world.add(Some(subchild));
    

    let subchild = world.add(Some(child));
    world.set_column_start(subchild, 0);
    world.set_row_start(subchild, 1);

    // let child = world.add(Some(node));
    // world.set_width(child, Units::Stretch(1.0));
    // world.set_height(child, Units::Auto);
    // world.set_layout_type(child, LayoutType::Row);
    // world.set_max_width(child, Units::Auto);

    // let subchild = world.add(Some(child));
    // world.set_width(subchild, Units::Pixels(100.0));
    // world.set_height(subchild, Units::Pixels(50.0));

    // let subchild = world.add(Some(child));
    // world.set_width(subchild, Units::Pixels(100.0));
    // world.set_height(subchild, Units::Pixels(50.0));

    // let subchild = world.add(Some(child));
    // world.set_width(subchild, Units::Pixels(100.0));
    // world.set_height(subchild, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    render(world, root);
}
