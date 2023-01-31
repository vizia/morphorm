use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_left_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Pixels(50.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_left_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Percentage(50.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn stretch_left_pixels_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_left_percentage_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Percentage(25.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Pixels(50.0));


    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 150.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 150.0, height: 150.0 }));
}

#[test]
fn percentage_left_percentage_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Percentage(25.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Percentage(50.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 150.0 }));
}

#[test]
fn stretch_left_percentage_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Percentage(25.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 450.0, posy: 0.0, width: 150.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 450.0, posy: 0.0, width: 150.0, height: 150.0 }));
}

#[test]
fn pixels_left_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Pixels(50.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 550.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 550.0, height: 150.0 }));
}

#[test]
fn percentage_left_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Percentage(50.0));    

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));
}

#[test]
fn stretch_left_stretch_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));
}

#[test]
fn pixels_left_self_directed() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Pixels(50.0));
    world.set_position_type(node, PositionType::SelfDirected);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_left_self_directed() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Percentage(50.0));
    world.set_position_type(node, PositionType::SelfDirected);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn stretch_left_self_directed() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));
    world.set_position_type(node, PositionType::SelfDirected);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));
}
