use morphorm::*;
use morphorm_ecs::*;

#[test]
fn border_pixels_stretch_child() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_border(root, Units::Pixels(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Stretch(1.0));
    world.set_height(node, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 50.0, width: 500.0, height: 500.0 }));
}

#[test]
fn border_pixels_stretch_child2() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_border(root, Units::Pixels(50.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Stretch(1.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Stretch(1.0));
    world.set_height(node2, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 50.0, posy: 50.0, width: 500.0, height: 250.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 50.0, posy: 300.0, width: 500.0, height: 250.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 50.0, posy: 50.0, width: 250.0, height: 500.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 300.0, posy: 50.0, width: 250.0, height: 500.0 }));
}