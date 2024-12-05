use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_padding_left_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_left(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 20.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 20.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_padding_left_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_left(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_padding_top_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_top(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 20.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 20.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_padding_top_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_top(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 300.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_padding_right_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_right(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));
    world.set_right(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_padding_right_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_right(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Stretch(1.0));
    world.set_right(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_padding_bottom_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_bottom(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_top(node, Units::Stretch(1.0));
    world.set_bottom(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_padding_bottom_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_bottom(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_padding_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_left(root, Units::Pixels(20.0));
    world.set_padding_top(root, Units::Pixels(20.0));
    world.set_padding_right(root, Units::Pixels(20.0));
    world.set_padding_bottom(root, Units::Pixels(20.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);
    world.set_top(node, Units::Auto);
    world.set_right(node, Units::Auto);
    world.set_bottom(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 20.0, posy: 20.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 20.0, posy: 20.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_padding_pixels_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_padding_left(root, Units::Percentage(50.0));
    world.set_padding_top(root, Units::Percentage(50.0));
    world.set_padding_right(root, Units::Percentage(50.0));
    world.set_padding_bottom(root, Units::Percentage(50.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(150.0));
    world.set_left(node, Units::Auto);
    world.set_top(node, Units::Auto);
    world.set_right(node, Units::Auto);
    world.set_bottom(node, Units::Auto);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 300.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 300.0, width: 100.0, height: 150.0 }));
}
