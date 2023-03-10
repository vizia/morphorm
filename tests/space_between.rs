use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_col_between() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_col_between(root, Units::Pixels(20.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 120.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_col_between() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_col_between(root, Units::Percentage(50.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 400.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}

#[test]
fn stretch_col_between() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_col_between(root, Units::Stretch(1.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_row_between() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_row_between(root, Units::Pixels(20.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 170.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_row_between() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_row_between(root, Units::Percentage(50.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 450.0, width: 100.0, height: 150.0 }));
}

#[test]
fn stretch_row_between() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_row_between(root, Units::Stretch(1.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 450.0, width: 100.0, height: 150.0 }));
}
