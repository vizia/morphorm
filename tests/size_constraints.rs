use morphorm::*;
use morphorm_ecs::*;

#[test]
fn min_width_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_min_width(node, Units::Pixels(200.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 100.0 }));
}

#[test]
fn max_width_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_max_width(node, Units::Pixels(200.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 100.0 }));
}

#[test]
fn min_height_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_min_height(node, Units::Pixels(200.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 200.0 }));
}

#[test]
fn max_height_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(400.0));
    world.set_max_height(node, Units::Pixels(200.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 200.0 }));
}

#[test]
fn min_width_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_min_width(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 100.0 }));
}

#[test]
fn max_width_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_max_width(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 100.0 }));
}

#[test]
fn min_height_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_min_height(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 300.0 }));
}

#[test]
fn max_height_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(400.0));
    world.set_max_height(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 300.0 }));
}

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

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

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

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

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

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

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

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

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

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

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

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

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

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

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

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 300.0 }));
}
