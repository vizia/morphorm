use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_cross_before_pixels_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_main(node, Units::Pixels(100.0));
    world.set_cross(node, Units::Pixels(150.0));
    world.set_cross_before(node, Units::Pixels(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 0.0, posy: 50.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 50.0, posy: 0.0, width: 150.0, height: 100.0 })
    );
}

#[test]
fn percentage_cross_before_pixels_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_main(node, Units::Pixels(100.0));
    world.set_cross(node, Units::Pixels(150.0));
    world.set_cross_before(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 100.0 })
    );
}

#[test]
fn stretch_cross_before_pixels_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_main(node, Units::Pixels(100.0));
    world.set_cross(node, Units::Pixels(150.0));
    world.set_cross_before(node, Units::Stretch(1.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 0.0, posy: 450.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 450.0, posy: 0.0, width: 150.0, height: 100.0 })
    );
}

#[test]
fn pixels_cross_before_percentage_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_cross(node, Units::Percentage(25.0));
    world.set_main(node, Units::Pixels(150.0));
    world.set_cross_before(node, Units::Pixels(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 0.0, posy: 50.0, width: 150.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 50.0, posy: 0.0, width: 150.0, height: 150.0 })
    );
}

#[test]
fn percentage_cross_before_percentage_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_cross(node, Units::Percentage(25.0));
    world.set_main(node, Units::Pixels(150.0));
    world.set_cross_before(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 0.0, posy: 300.0, width: 150.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 150.0 })
    );
}

#[test]
fn stretch_cross_before_percentage_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_cross(node, Units::Percentage(25.0));
    world.set_main(node, Units::Pixels(150.0));
    world.set_cross_before(node, Units::Stretch(1.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 0.0, posy: 450.0, width: 150.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 450.0, posy: 0.0, width: 150.0, height: 150.0 })
    );
}

#[test]
fn pixels_cross_before_stretch_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_cross(node, Units::Stretch(1.0));
    world.set_main(node, Units::Pixels(150.0));
    world.set_cross_before(node, Units::Pixels(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 0.0, posy: 50.0, width: 150.0, height: 550.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 50.0, posy: 0.0, width: 550.0, height: 150.0 })
    );
}

#[test]
fn percentage_cross_before_stretch_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_cross(node, Units::Stretch(1.0));
    world.set_main(node, Units::Pixels(150.0));
    world.set_cross_before(node, Units::Percentage(50.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 0.0, posy: 300.0, width: 150.0, height: 300.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 })
    );
}

#[test]
fn stretch_cross_before_stretch_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_cross(node, Units::Stretch(1.0));
    world.set_main(node, Units::Pixels(150.0));
    world.set_cross_before(node, Units::Stretch(1.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 0.0, posy: 300.0, width: 150.0, height: 300.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node),
        Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 })
    );
}