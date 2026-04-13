use morphorm::*;
use morphorm_ecs::*;

#[test]
fn rtl_reverses_row_order() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_layout_type(root, LayoutType::Row);
    world.set_direction(root, Direction::RightToLeft);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(100.0));

    let first = world.add(Some(root));
    world.set_width(first, Units::Pixels(50.0));
    world.set_height(first, Units::Pixels(50.0));

    let second = world.add(Some(root));
    world.set_width(second, Units::Pixels(50.0));
    world.set_height(second, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(first), Some(&Rect { posx: 250.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(second), Some(&Rect { posx: 200.0, posy: 0.0, width: 50.0, height: 50.0 }));
}

#[test]
fn rtl_swaps_row_padding_left_and_right() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_layout_type(root, LayoutType::Row);
    world.set_direction(root, Direction::RightToLeft);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(100.0));
    world.set_padding_left(root, Units::Pixels(20.0));
    world.set_padding_right(root, Units::Pixels(40.0));

    let child = world.add(Some(root));
    world.set_width(child, Units::Pixels(50.0));
    world.set_height(child, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 230.0, posy: 0.0, width: 50.0, height: 50.0 }));
}

#[test]
fn rtl_reverses_row_alignment_horizontally() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_layout_type(root, LayoutType::Row);
    world.set_direction(root, Direction::RightToLeft);
    world.set_alignment(root, Alignment::TopRight);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(100.0));

    let child = world.add(Some(root));
    world.set_width(child, Units::Pixels(50.0));
    world.set_height(child, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
}

#[test]
fn rtl_reverses_column_alignment_horizontally() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_layout_type(root, LayoutType::Column);
    world.set_direction(root, Direction::RightToLeft);
    world.set_alignment(root, Alignment::TopRight);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(100.0));

    let child = world.add(Some(root));
    world.set_width(child, Units::Pixels(50.0));
    world.set_height(child, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
}
