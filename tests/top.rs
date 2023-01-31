use morphorm::*;
use morphorm_ecs::*;

#[test]
fn stretch_cross_space_pixels_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Row);

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_top(node, Units::Stretch(1.0));
    world.set_bottom(node, Units::Stretch(1.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 250.0, width: 100.0, height: 100.0 }));
}

#[test]
fn stretch_cross_space_percentage_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Row);

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Percentage(50.0));
    world.set_top(node, Units::Stretch(1.0));
    world.set_bottom(node, Units::Stretch(1.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 300.0 }));
}

#[test]
fn stretch_cross_space_stretch_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Row);

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Stretch(1.0));
    world.set_top(node, Units::Stretch(1.0));
    world.set_bottom(node, Units::Stretch(1.0));

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 200.0, width: 100.0, height: 200.0 }));
}
