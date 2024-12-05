use morphorm::*;
use morphorm_ecs::*;

#[test]
fn min_width_pixels_min_height_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_min_width(node, Units::Pixels(200.0));
    world.set_min_height(node, Units::Pixels(200.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 }));
}

#[test]
fn max_width_pixels_max_height_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Pixels(400.0));
    world.set_max_width(node, Units::Pixels(200.0));
    world.set_max_height(node, Units::Pixels(200.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 }));
}

#[test]
fn min_width_pixels_max_height_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(400.0));
    world.set_min_width(node, Units::Pixels(200.0));
    world.set_max_height(node, Units::Pixels(200.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 }));
}

#[test]
fn max_width_pixels_min_height_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_max_width(node, Units::Pixels(200.0));
    world.set_min_height(node, Units::Pixels(200.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 }));
}

#[test]
fn min_width_percentage_min_height_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_min_width(node, Units::Percentage(50.0));
    world.set_min_height(node, Units::Percentage(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn max_width_percentage_max_height_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Pixels(400.0));
    world.set_max_width(node, Units::Percentage(50.0));
    world.set_max_height(node, Units::Percentage(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_width_percentage_max_height_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(400.0));
    world.set_min_width(node, Units::Percentage(50.0));
    world.set_max_height(node, Units::Percentage(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn max_width_percentage_min_height_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_max_width(node, Units::Percentage(50.0));
    world.set_min_height(node, Units::Percentage(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_width_percentage_width_auto() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);
    world.set_min_width(node, Units::Percentage(100.0));

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(200.0));
    world.set_height(node2, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 100.0 }));
}

#[test]
fn min_width_auto() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_min_width(node, Units::Auto);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(300.0));
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 150.0, posy: 200.0, width: 300.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_height_auto() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_min_height(node, Units::Auto);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(300.0));
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 200.0, posy: 150.0, width: 200.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_size_auto() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_min_width(node, Units::Auto);
    world.set_min_height(node, Units::Auto);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(300.0));
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 150.0, posy: 150.0, width: 300.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_width_auto_absolute() {
    let mut world = World::default();
    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_min_width(node, Units::Auto);
    world.set_position_type(node, PositionType::Absolute);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(300.0));
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());
    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_height_auto_absolute() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_min_height(node, Units::Auto);
    world.set_position_type(node, PositionType::Absolute);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(300.0));
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_size_auto_absolute() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_min_width(node, Units::Auto);
    world.set_min_height(node, Units::Auto);
    world.set_position_type(node, PositionType::Absolute);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(300.0));
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_width_auto_child_absolute() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_min_width(node, Units::Auto);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(300.0));
    world.set_position_type(node2, PositionType::Absolute);
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 200.0, posy: 200.0, width: 200.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_height_auto_child_absolute() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_min_height(node, Units::Auto);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(300.0));
    world.set_position_type(node2, PositionType::Absolute);
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 200.0, posy: 200.0, width: 200.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}

#[test]
fn min_size_auto_child_absolute() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding(root, Units::Pixels(200.0));
    world.set_alignment(root, Alignment::Center);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_min_width(node, Units::Auto);
    world.set_min_height(node, Units::Auto);

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Pixels(300.0));
    world.set_height(node2, Units::Pixels(300.0));
    world.set_position_type(node2, PositionType::Absolute);
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 200.0, posy: 200.0, width: 200.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}
