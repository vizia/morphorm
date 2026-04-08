mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(20.0));
    world.set_layout_type(root, LayoutType::Column);
    world.set_vertical_gap(root, Units::Pixels(20.0));
    world.set_alignment(root, Alignment::TopLeft);

    // Row wrapping example
    let row_wrap_container = world.add(Some(root));
    world.set_width(row_wrap_container, Units::Stretch(1.0));
    world.set_height(row_wrap_container, Units::Auto);
    world.set_layout_type(row_wrap_container, LayoutType::Row);
    world.set_wrap(row_wrap_container, LayoutWrap::Wrap);
    world.set_horizontal_gap(row_wrap_container, Units::Pixels(10.0));
    world.set_vertical_gap(row_wrap_container, Units::Pixels(10.0));
    world.set_alignment(row_wrap_container, Alignment::TopLeft);

    // Add items to row wrap container
    for _i in 0..6 {
        let item = world.add(Some(row_wrap_container));
        world.set_width(item, Units::Pixels(80.0));
        world.set_height(item, Units::Pixels(60.0));
    }

    // Column wrapping example
    let col_wrap_container = world.add(Some(root));
    world.set_width(col_wrap_container, Units::Pixels(200.0));
    world.set_height(col_wrap_container, Units::Pixels(250.0));
    world.set_layout_type(col_wrap_container, LayoutType::Column);
    world.set_wrap(col_wrap_container, LayoutWrap::Wrap);
    world.set_horizontal_gap(col_wrap_container, Units::Pixels(10.0));
    world.set_vertical_gap(col_wrap_container, Units::Pixels(10.0));
    world.set_alignment(col_wrap_container, Alignment::TopLeft);

    // Add items to column wrap container
    for _i in 0..6 {
        let item = world.add(Some(col_wrap_container));
        world.set_width(item, Units::Pixels(60.0));
        world.set_height(item, Units::Pixels(50.0));
    }

    // RTL wrapping example
    let rtl_wrap_container = world.add(Some(root));
    world.set_width(rtl_wrap_container, Units::Pixels(300.0));
    world.set_height(rtl_wrap_container, Units::Auto);
    world.set_layout_type(rtl_wrap_container, LayoutType::Row);
    world.set_direction(rtl_wrap_container, Direction::RightToLeft);
    world.set_wrap(rtl_wrap_container, LayoutWrap::Wrap);
    world.set_horizontal_gap(rtl_wrap_container, Units::Pixels(10.0));
    world.set_vertical_gap(rtl_wrap_container, Units::Pixels(10.0));
    world.set_alignment(rtl_wrap_container, Alignment::TopLeft);

    // Add items to RTL wrap container
    for _i in 0..4 {
        let item = world.add(Some(rtl_wrap_container));
        world.set_width(item, Units::Pixels(100.0));
        world.set_height(item, Units::Pixels(50.0));
    }

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    render(world, root);
}
