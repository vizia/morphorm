use morphorm::*;
use morphorm_ecs::*;

#[test]
fn relative_children_overlap_in_overlay() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Overlay);

    let child1 = world.add(Some(root));
    world.set_width(child1, Units::Pixels(100.0));
    world.set_height(child1, Units::Pixels(50.0));

    let child2 = world.add(Some(root));
    world.set_width(child2, Units::Pixels(80.0));
    world.set_height(child2, Units::Pixels(40.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 40.0 }));
}

#[test]
fn overlay_alignment_centers_children_independently() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);
    world.set_layout_type(root, LayoutType::Overlay);

    let child = world.add(Some(root));
    world.set_width(child, Units::Pixels(100.0));
    world.set_height(child, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 100.0, posy: 75.0, width: 100.0, height: 50.0 }));
}

#[test]
fn overlay_padding_offsets_relative_children() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Overlay);
    world.set_padding_left(root, Units::Pixels(10.0));
    world.set_padding_right(root, Units::Pixels(20.0));
    world.set_padding_top(root, Units::Pixels(30.0));
    world.set_padding_bottom(root, Units::Pixels(40.0));

    let child = world.add(Some(root));
    world.set_width(child, Units::Pixels(100.0));
    world.set_height(child, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 10.0, posy: 30.0, width: 100.0, height: 50.0 }));
}

#[test]
fn overlay_auto_size_uses_max_child_extent() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Row);

    let overlay = world.add(Some(root));
    world.set_width(overlay, Units::Auto);
    world.set_height(overlay, Units::Auto);
    world.set_alignment(overlay, Alignment::TopLeft);
    world.set_layout_type(overlay, LayoutType::Overlay);
    world.set_padding_left(overlay, Units::Pixels(10.0));
    world.set_padding_right(overlay, Units::Pixels(30.0));
    world.set_padding_top(overlay, Units::Pixels(20.0));
    world.set_padding_bottom(overlay, Units::Pixels(40.0));

    let child1 = world.add(Some(overlay));
    world.set_width(child1, Units::Pixels(100.0));
    world.set_height(child1, Units::Pixels(50.0));

    let child2 = world.add(Some(overlay));
    world.set_width(child2, Units::Pixels(60.0));
    world.set_height(child2, Units::Pixels(120.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(overlay), Some(&Rect { posx: 0.0, posy: 0.0, width: 140.0, height: 180.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 10.0, posy: 20.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 10.0, posy: 20.0, width: 60.0, height: 120.0 }));
}

#[test]
fn overlay_rtl_flips_absolute_child_horizontal_offset() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_layout_type(root, LayoutType::Overlay);
    // RightToLeft: `left` becomes the trailing edge, so a `left: 20px` absolute child
    // should be placed 20px from the *right* edge (x = 300 - 20 - 60 = 220).
    world.set_direction(root, Direction::RightToLeft);

    let absolute = world.add(Some(root));
    world.set_position_type(absolute, PositionType::Absolute);
    world.set_width(absolute, Units::Pixels(60.0));
    world.set_height(absolute, Units::Pixels(40.0));
    world.set_left(absolute, Units::Pixels(20.0));
    world.set_top(absolute, Units::Pixels(10.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Under RTL `left` is the trailing edge, so x = parent_width - left - child_width.
    assert_eq!(world.cache.bounds(absolute), Some(&Rect { posx: 220.0, posy: 10.0, width: 60.0, height: 40.0 }));
}

#[test]
fn overlay_preserves_absolute_child_positioning() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Overlay);

    let relative = world.add(Some(root));
    world.set_width(relative, Units::Pixels(50.0));
    world.set_height(relative, Units::Pixels(50.0));

    let absolute = world.add(Some(root));
    world.set_position_type(absolute, PositionType::Absolute);
    world.set_width(absolute, Units::Pixels(60.0));
    world.set_height(absolute, Units::Pixels(70.0));
    world.set_left(absolute, Units::Pixels(20.0));
    world.set_top(absolute, Units::Pixels(30.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(relative), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(absolute), Some(&Rect { posx: 20.0, posy: 30.0, width: 60.0, height: 70.0 }));
}
