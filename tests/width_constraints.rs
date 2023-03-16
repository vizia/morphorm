use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_min_width_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_min_width(node, Units::Pixels(200.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 100.0 }));
}

#[test]
fn pixels_max_width_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_max_width(node, Units::Pixels(200.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 100.0 }));
}

#[test]
fn percentage_min_width_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_min_width(node, Units::Percentage(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 100.0 }));
}

#[test]
fn percentage_max_width_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_max_width(node, Units::Percentage(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 100.0 }));
}

#[test]
fn stretch_min_width_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_min_width(node, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 100.0 }));
}

#[test]
fn stretch_max_width_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_max_width(node, Units::Stretch(0.5));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 100.0 }));
}

#[test]
fn pixels_min_width_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Pixels(400.0));
    world.set_min_width(node, Units::Pixels(700.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 700.0, height: 400.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 700.0, height: 400.0 }));
}

#[test]
fn percentage_min_width_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Pixels(400.0));
    world.set_min_width(node, Units::Percentage(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 900.0, height: 400.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 900.0, height: 400.0 }));
}
