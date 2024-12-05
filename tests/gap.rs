use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_horizontal_gap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_horizontal_gap(root, Units::Pixels(20.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 120.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_horizontal_gap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_horizontal_gap(root, Units::Percentage(50.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 400.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}

#[test]
fn stretch_horizontal_gap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_horizontal_gap(root, Units::Stretch(1.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 150.0, width: 100.0, height: 150.0 }));
}

#[test]
fn pixels_vertical_gap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_vertical_gap(root, Units::Pixels(20.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 170.0, width: 100.0, height: 150.0 }));
}

#[test]
fn percentage_vertical_gap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_vertical_gap(root, Units::Percentage(50.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 450.0, width: 100.0, height: 150.0 }));
}

#[test]
fn stretch_vertical_gap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_vertical_gap(root, Units::Stretch(1.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(150.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 450.0, width: 100.0, height: 150.0 }));
}
