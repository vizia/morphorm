use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_width_pixels_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_width_pixels_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Percentage(50.0));
    world.set_height(node, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));
}

#[test]
fn stretch_width_pixels_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 150.0 }));
}

#[test]
fn percentage_width_percentage_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Percentage(50.0));
    world.set_height(node, Units::Percentage(25.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));
}

#[test]
fn stretch_width_percentage_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Percentage(25.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 150.0 }));
}

#[test]
fn stretch_width_stretch_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 600.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 600.0 }));
}

#[test]
fn auto_width_pixels_child() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn auto_width_pixels_child_self_directed() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_position_type(node2, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn auto_width_pixels_children_self_directed() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_position_type(node2, PositionType::SelfDirected);

    let node3 = world.add(Some(node1));
    world.set_width(node3, Units::Pixels(200.0));
    world.set_height(node3, Units::Pixels(100.0));
    world.set_position_type(node3, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));
}

#[test]
fn auto_width_pixels_children_self_directed_with_pixels_left() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_position_type(node2, PositionType::SelfDirected);

    let node3 = world.add(Some(node1));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(100.0));
    world.set_left(node3, Units::Pixels(100.0));
    world.set_position_type(node3, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));
}

#[test]
fn auto_width_single_child_with_pixels_left() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_left(node2, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));
}

#[test]
fn auto_width_single_child_with_percentage_left() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_left(node2, Units::Percentage(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn auto_width_single_child_with_stretch_left() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_left(node2, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn auto_width_single_child_with_pixels_right() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_right(node2, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));
}

#[test]
fn auto_width_single_child_with_percentage_right() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_right(node2, Units::Percentage(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn auto_width_single_child_with_stretch_right() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_right(node2, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn auto_width_single_child_with_pixels_left_with_pixels_right() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_left(node2, Units::Pixels(100.0));
    world.set_right(node2, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));
}

#[test]
fn auto_width_single_child_with_stretch_left_with_stretch_right() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));
    world.set_left(node2, Units::Stretch(1.0));
    world.set_right(node2, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn auto_width_multiple_children() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(150.0));
    world.set_layout_type(node1, LayoutType::Row);

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(100.0));

    let node3 = world.add(Some(node1));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 150.0 }));

    world.set_layout_type(node1, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}
