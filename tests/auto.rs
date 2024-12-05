use morphorm::*;
use morphorm_ecs::*;

#[test]
fn auto_min_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);

    let child1 = world.add(Some(node));
    world.set_width(child1, Units::Stretch(1.0));
    world.set_min_width(child1, Units::Auto);
    world.set_height(child1, Units::Pixels(50.0));
    world.set_layout_type(child1, LayoutType::Row);
    world.set_content_size(child1, |_, _, height| {
        let height = height.unwrap();
        (50.0, height)
    });

    let child2 = world.add(Some(node));
    world.set_width(child2, Units::Stretch(1.0));
    world.set_min_width(child2, Units::Auto);
    world.set_height(child2, Units::Pixels(50.0));
    world.set_layout_type(child2, LayoutType::Row);
    world.set_content_size(child2, |_, _, height| {
        let height = height.unwrap();
        (80.0, height)
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 50.0, width: 80.0, height: 50.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 50.0, width: 80.0, height: 50.0 }));
}

#[test]
fn auto_min_width2() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);

    let child1 = world.add(Some(node));
    world.set_width(child1, Units::Stretch(1.0));
    world.set_min_width(child1, Units::Auto);
    world.set_height(child1, Units::Pixels(50.0));
    // world.set_layout_type(child, LayoutType::Row);

    let subchild1 = world.add(Some(child1));
    world.set_width(subchild1, Units::Stretch(1.0));
    world.set_min_width(subchild1, Units::Auto);
    world.set_height(subchild1, Units::Pixels(50.0));
    world.set_layout_type(subchild1, LayoutType::Row);
    world.set_content_size(subchild1, |_, _, height| {
        let height = height.unwrap();
        (50.0, height)
    });

    let child2 = world.add(Some(node));
    world.set_width(child2, Units::Stretch(1.0));
    world.set_min_width(child2, Units::Auto);
    world.set_height(child2, Units::Pixels(50.0));
    // world.set_layout_type(child, LayoutType::Row);

    let subchild2 = world.add(Some(child2));
    world.set_width(subchild2, Units::Stretch(1.0));
    world.set_min_width(subchild2, Units::Auto);
    world.set_height(subchild2, Units::Pixels(50.0));
    world.set_layout_type(subchild2, LayoutType::Row);
    world.set_content_size(subchild2, |_, _, height| {
        let height = height.unwrap();
        (80.0, height)
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 50.0, width: 80.0, height: 50.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 50.0, width: 80.0, height: 50.0 }));
}

#[test]
fn auto_min_width3() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Pixels(100.0));

    let child1 = world.add(Some(node));
    world.set_width(child1, Units::Stretch(1.0));
    world.set_min_width(child1, Units::Auto);
    world.set_height(child1, Units::Stretch(1.0));
    world.set_layout_type(child1, LayoutType::Row);
    world.set_content_size(child1, |_, _, height| {
        let height = height.unwrap();
        (50.0, height)
    });

    let child2 = world.add(Some(node));
    world.set_width(child2, Units::Stretch(1.0));
    world.set_min_width(child2, Units::Auto);
    world.set_height(child2, Units::Stretch(1.0));
    world.set_layout_type(child2, LayoutType::Row);
    world.set_content_size(child2, |_, _, height| {
        let height = height.unwrap();
        (80.0, height)
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 50.0, width: 80.0, height: 50.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 0.0, posy: 50.0, width: 80.0, height: 50.0 }));
}

#[test]
fn auto_min_width4() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Auto);
    world.set_layout_type(node, LayoutType::Row);

    let child1 = world.add(Some(node));
    world.set_width(child1, Units::Stretch(1.0));
    world.set_min_height(child1, Units::Auto);
    world.set_height(child1, Units::Stretch(1.0));
    world.set_layout_type(child1, LayoutType::Row);
    world.set_content_size(child1, |_, width, _| {
        let width = width.unwrap();
        (width, 50.0)
    });

    let child2 = world.add(Some(node));
    world.set_width(child2, Units::Stretch(1.0));
    world.set_min_height(child2, Units::Auto);
    world.set_height(child2, Units::Stretch(1.0));
    world.set_layout_type(child2, LayoutType::Row);
    world.set_content_size(child2, |_, width, _| {
        let width = width.unwrap();
        (width, 80.0)
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 80.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 80.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 50.0, posy: 0.0, width: 50.0, height: 80.0 }));

    world.set_layout_type(root, LayoutType::Column);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 80.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 0.0, width: 50.0, height: 80.0 }));
    assert_eq!(world.cache.bounds(child2), Some(&Rect { posx: 50.0, posy: 0.0, width: 50.0, height: 80.0 }));
}

// #[test]
// fn percentage_left_pixels_width() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Pixels(100.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Percentage(50.0));

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));
// }

// #[test]
// fn stretch_left_pixels_width() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Pixels(100.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Stretch(1.0));

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));
// }

// #[test]
// fn pixels_left_percentage_width() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Percentage(25.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Pixels(50.0));

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 150.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 150.0, height: 150.0 }));
// }

// #[test]
// fn percentage_left_percentage_width() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Percentage(25.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Percentage(50.0));

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 150.0 }));
// }

// #[test]
// fn stretch_left_percentage_width() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Percentage(25.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Stretch(1.0));

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 450.0, posy: 0.0, width: 150.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 450.0, posy: 0.0, width: 150.0, height: 150.0 }));
// }

// #[test]
// fn pixels_left_stretch_width() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Stretch(1.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Pixels(50.0));

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 550.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 550.0, height: 150.0 }));
// }

// #[test]
// fn percentage_left_stretch_width() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Stretch(1.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Percentage(50.0));

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));
// }

// #[test]
// fn stretch_left_stretch_width() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Stretch(1.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Stretch(1.0));

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));
// }

// #[test]
// fn pixels_left_pixels_width_absolute() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Pixels(100.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Pixels(50.0));
//     world.set_position_type(node, PositionType::Absolute);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 100.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 100.0, height: 150.0 }));
// }

// #[test]
// fn percentage_left_pixels_width_absolute() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Pixels(100.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Percentage(50.0));
//     world.set_position_type(node, PositionType::Absolute);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 100.0, height: 150.0 }));
// }

// #[test]
// fn stretch_left_pixels_width_absolute() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Pixels(100.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Stretch(1.0));
//     world.set_position_type(node, PositionType::Absolute);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));
// }

// #[test]
// fn pixels_left_percentage_width_absolute() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Percentage(25.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Pixels(50.0));
//     world.set_position_type(node, PositionType::Absolute);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 150.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 150.0, height: 150.0 }));
// }

// #[test]
// fn percentage_left_percentage_width_absolute() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Percentage(25.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Percentage(50.0));
//     world.set_position_type(node, PositionType::Absolute);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 150.0 }));
// }

// #[test]
// fn stretch_left_percentage_width_absolute() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Percentage(25.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Stretch(1.0));
//     world.set_position_type(node, PositionType::Absolute);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 450.0, posy: 0.0, width: 150.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 450.0, posy: 0.0, width: 150.0, height: 150.0 }));
// }

// #[test]
// fn pixels_left_stretch_width_absolute() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Stretch(1.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Pixels(50.0));
//     world.set_position_type(node, PositionType::Absolute);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 550.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 50.0, posy: 0.0, width: 550.0, height: 150.0 }));
// }

// #[test]
// fn percentage_left_stretch_width_absolute() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Stretch(1.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Percentage(50.0));
//     world.set_position_type(node, PositionType::Absolute);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));
// }

// #[test]
// fn stretch_left_stretch_width_absolute() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_width(root, Units::Pixels(600.0));
//     world.set_height(root, Units::Pixels(600.0));

//     world.set_layout_type(root, LayoutType::Row);

//     let node = world.add(Some(root));
//     world.set_width(node, Units::Stretch(1.0));
//     world.set_height(node, Units::Pixels(150.0));
//     world.set_left(node, Units::Stretch(1.0));
//     world.set_position_type(node, PositionType::Absolute);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));

//     world.set_layout_type(root, LayoutType::Column);

//     root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

//     assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 }));
// }
