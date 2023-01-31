use morphorm::*;
use morphorm_ecs::*;

#[test]
fn min_left_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Pixels(50.0));
    world.set_min_left(node, Units::Pixels(100.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn min_left_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Pixels(50.0));
    world.set_min_left(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn max_left_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Pixels(500.0));
    world.set_max_left(node, Units::Pixels(100.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn max_left_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Pixels(500.0));
    world.set_max_left(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn min_top_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Pixels(50.0));
    world.set_min_top(node, Units::Pixels(100.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 100.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 100.0, width: 100.0, height: 150.0 }));
}

#[test]
fn min_top_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Pixels(50.0));
    world.set_min_top(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));
}

#[test]
fn max_top_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Pixels(500.0));
    world.set_max_top(node, Units::Pixels(100.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 100.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 100.0, width: 100.0, height: 150.0 }));
}

#[test]
fn max_top_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Pixels(500.0));
    world.set_max_top(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));
}