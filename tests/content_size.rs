use morphorm::*;
use morphorm_ecs::*;

#[test]
fn content_size_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Auto);
    world.set_content_size(node, |_, width, _| (width.unwrap(), 100.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 100.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 100.0 }));
}

#[test]
fn content_size_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Pixels(400.0));
    world.set_content_size(node, |_, _, height| (100.0, height.unwrap()));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 400.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 400.0 }));
}

#[test]
fn content_size_height2() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Auto);
    world.set_content_size(node1, |_, width, _| (width.unwrap(), width.unwrap() / 2.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Stretch(1.0));
    world.set_height(node2, Units::Auto);
    world.set_content_size(node2, |_, width, _| (width.unwrap(), width.unwrap() / 2.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 300.0, width: 600.0, height: 300.0 }));
}

#[test]
fn content_size_width2() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Stretch(1.0));
    world.set_content_size(node1, |_, _, height| (height.unwrap() / 2.0, height.unwrap()));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Auto);
    world.set_height(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |_, _, height| (height.unwrap() / 2.0, height.unwrap()));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 600.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 600.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 300.0, width: 150.0, height: 300.0 }));
}

#[test]
fn content_size_width_parent_auto_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Stretch(1.0));
    world.set_layout_type(node, LayoutType::Row);

    let node1 = world.add(Some(node));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Stretch(1.0));
    world.set_content_size(node1, |_, _, height| (height.unwrap() / 2.0, height.unwrap()));

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Auto);
    world.set_height(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |_, _, height| (height.unwrap() / 2.0, height.unwrap()));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 600.0 }));
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 600.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 600.0 }));

    world.set_layout_type(node, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 600.0 }));
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 300.0, width: 150.0, height: 300.0 }));
}

#[test]
fn content_size_height_parent_auto_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    let node = world.add(Some(root));
    world.set_height(node, Units::Auto);
    world.set_width(node, Units::Stretch(1.0));
    world.set_layout_type(node, LayoutType::Row);

    let node1 = world.add(Some(node));
    world.set_height(node1, Units::Auto);
    world.set_width(node1, Units::Stretch(1.0));
    world.set_content_size(node1, |_, width, _| (width.unwrap(), width.unwrap() / 2.0));

    let node2 = world.add(Some(node));
    world.set_height(node2, Units::Auto);
    world.set_width(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |_, width, _| (width.unwrap(), width.unwrap() / 2.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 150.0 }));
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));

    world.set_layout_type(node, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 600.0 }));
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 300.0, width: 600.0, height: 300.0 }));
}

#[test]
fn content_size_width_parent_auto_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    let node = world.add(Some(root));
    world.set_height(node, Units::Auto);
    world.set_width(node, Units::Stretch(1.0));
    world.set_layout_type(node, LayoutType::Row);

    let node1 = world.add(Some(node));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Stretch(1.0));
    world.set_content_size(node1, |_, _, height| (height.unwrap() / 2.0, height.unwrap()));

    let node2 = world.add(Some(node));
    world.set_width(node2, Units::Auto);
    world.set_height(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |_, _, height| (height.unwrap() / 2.0, height.unwrap()));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 0.0 }));
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));

    world.set_layout_type(node, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 0.0 }));
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));
}

#[test]
fn content_size_height_parent_auto_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Stretch(1.0));
    world.set_layout_type(node, LayoutType::Row);
    world.set_alignment(node, Alignment::TopLeft);

    let node1 = world.add(Some(node));
    world.set_height(node1, Units::Auto);
    world.set_width(node1, Units::Stretch(1.0));
    world.set_alignment(node1, Alignment::TopLeft);
    world.set_content_size(node1, |_, width, _| (width.unwrap(), width.unwrap() / 2.0));

    let node2 = world.add(Some(node));
    world.set_height(node2, Units::Auto);
    world.set_width(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |_, width, _| (width.unwrap(), width.unwrap() / 2.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 600.0 }));
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));

    world.set_layout_type(node, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 600.0 }));
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));
}

#[test]
fn nested_content_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Auto);
    world.set_height(node1, Units::Pixels(200.0));
    world.set_layout_type(node1, LayoutType::Row);
    world.set_alignment(node1, Alignment::TopLeft);

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Auto);
    world.set_height(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |_, _, height| (height.unwrap(), height.unwrap()));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 }));

    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 }));
}

#[test]
fn content_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    world.set_layout_type(root, LayoutType::Column);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);
    world.set_layout_type(node, LayoutType::Row);
    world.set_content_size(node, |_, _, _| (100.0, 50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));

    world.set_layout_type(node, LayoutType::Row);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));
}

#[test]
fn equal_aspect_ratio() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);

    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Stretch(1.0));
    world.set_content_size(node, |_, _, height| (height.unwrap(), height.unwrap()));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 600.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 600.0 }));
}
