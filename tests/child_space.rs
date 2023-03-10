use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_child_left_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_left(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 20.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 20.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_child_left_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_left(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn stretch_child_left_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_left(root, Units::Stretch(1.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_child_top_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_top(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 20.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 20.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_child_top_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_top(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));
}

#[test]
fn stretch_child_top_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_top(root, Units::Stretch(1.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 450.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 450.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_child_right_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_right(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));
    world.set_right(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 480.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 480.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_child_right_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_right(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));
    world.set_right(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 200.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 200.0, posy: 0.0, width: 100.0, height: 150.0 }));
}


#[test]
fn stretch_child_right_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_right(root, Units::Stretch(1.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));
    world.set_right(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 250.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 250.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_child_bottom_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_bottom(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Stretch(1.0));
    world.set_bottom(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 430.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 430.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_child_bottom_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_bottom(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Stretch(1.0));
    world.set_bottom(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}


#[test]
fn stretch_child_bottom_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_bottom(root, Units::Stretch(1.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Stretch(1.0));
    world.set_bottom(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 225.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 225.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_child_space_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_left(root, Units::Pixels(20.0));
    world.set_child_top(root, Units::Pixels(20.0));
    world.set_child_right(root, Units::Pixels(20.0));
    world.set_child_bottom(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);
    world.set_top(node, Units::Auto);
    world.set_right(node, Units::Auto);
    world.set_bottom(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 20.0, posy: 20.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 20.0, posy: 20.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_child_space_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_left(root, Units::Percentage(50.0));
    world.set_child_top(root, Units::Percentage(50.0));
    world.set_child_right(root, Units::Percentage(50.0));
    world.set_child_bottom(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);
    world.set_top(node, Units::Auto);
    world.set_right(node, Units::Auto);
    world.set_bottom(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 300.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 300.0, width: 100.0, height: 150.0 }));
}

#[test]
fn stretch_child_space_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_left(root, Units::Stretch(1.0));
    world.set_child_top(root, Units::Stretch(1.0));
    world.set_child_right(root, Units::Stretch(1.0));
    world.set_child_bottom(root, Units::Stretch(1.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);
    world.set_top(node, Units::Auto);
    world.set_right(node, Units::Auto);
    world.set_bottom(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 250.0, posy: 225.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 250.0, posy: 225.0, width: 100.0, height: 150.0 }));
}