use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_right_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 150.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 })
    );
}

#[test]
fn percentage_right_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Percentage(25.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 250.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 })
    );
}

#[test]
fn stretch_right_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Stretch(1.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 })
    );
}

#[test]
fn pixels_right_percentage_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Percentage(25.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 200.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 })
    );
}

#[test]
fn percentage_right_percentage_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Percentage(25.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Percentage(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 450.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 })
    );
}

#[test]
fn stretch_right_percentage_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Percentage(25.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Stretch(1.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 })
    );
}

#[test]
fn pixels_right_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 450.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 550.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 })
    );
}

#[test]
fn percentage_right_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Percentage(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 })
    );
}

#[test]
fn stretch_right_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Stretch(1.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 250.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 })
    );
}
