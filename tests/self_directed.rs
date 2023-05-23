use morphorm::*;
use morphorm_ecs::*;

#[test]
fn self_directed_pixels_width_pixels_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_position_type(node, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 100.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 100.0 }));
}

#[test]
fn self_directed_pixels_width_percentage_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Percentage(25.0));
    world.set_position_type(node, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn self_directed_pixels_width_stretch_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_position_type(node, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 600.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 600.0 }));
}

#[test]
fn self_directed_pixels_width_auto_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Auto);
    world.set_position_type(node, PositionType::SelfDirected);

    let child = world.add(Some(node));
    world.set_width(child, Units::Pixels(50.0));
    world.set_height(child, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
}

#[test]
fn self_directed_percentage_width_pixels_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Percentage(50.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_position_type(node, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 100.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 100.0 }));
}

#[test]
fn self_directed_stretch_width_pixels_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_position_type(node, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 100.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 100.0 }));
}

#[test]
fn self_directed_auto_width_pixels_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Pixels(100.0));
    world.set_position_type(node, PositionType::SelfDirected);

    let child = world.add(Some(node));
    world.set_width(child, Units::Pixels(50.0));
    world.set_height(child, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
}

#[test]
fn self_directed_auto_width_auto_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);
    world.set_position_type(node, PositionType::SelfDirected);

    let child = world.add(Some(node));
    world.set_width(child, Units::Pixels(50.0));
    world.set_height(child, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
}

#[test]
fn self_directed_stretch_width_stretch_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_position_type(node, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 600.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 600.0 }));
}

#[test]
fn self_directed_percentage_width_percentage_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Percentage(50.0));
    world.set_height(node, Units::Percentage(25.0));
    world.set_position_type(node, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));
}

#[test]
fn auto_parent_pixels_child_stretch_self_directed_child() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);

    let child1 = world.add(Some(node));
    world.set_width(child1, Units::Pixels(50.0));
    world.set_height(child1, Units::Pixels(50.0));

    let child2 = world.add(Some(node));
    world.set_width(child2, Units::Stretch(1.0));
    world.set_height(child2, Units::Stretch(1.0));
    world.set_position_type(child2, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
}

#[test]
fn auto_parent_pixels_child_percentage_self_directed_child() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);

    let child1 = world.add(Some(node));
    world.set_width(child1, Units::Pixels(50.0));
    world.set_height(child1, Units::Pixels(50.0));

    let child2 = world.add(Some(node));
    world.set_width(child2, Units::Percentage(50.0));
    world.set_height(child2, Units::Percentage(25.0));
    world.set_position_type(child2, PositionType::SelfDirected);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 0.0, width: 25.0, height: 13.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 0.0, width: 25.0, height: 13.0 }));
}
