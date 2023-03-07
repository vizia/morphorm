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

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

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

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

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

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

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

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

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

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 100.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

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

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

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

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 100.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

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

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));
}

#[test]
fn min_right_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Pixels(50.0));
    world.set_min_right(node1, Units::Pixels(100.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 200.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}

#[test]
fn min_right_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Pixels(50.0));
    world.set_min_right(node1, Units::Percentage(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 400.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}

#[test]
fn max_right_pixels() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Pixels(400.0));
    world.set_max_right(node1, Units::Pixels(100.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 200.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}

#[test]
fn max_right_percentage() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));
    world.set_right(node1, Units::Pixels(400.0));
    world.set_max_right(node1, Units::Percentage(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 400.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}
