use morphorm::*;
use morphorm_ecs::*;

#[test]
fn pixels_main_after_pixels_main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_main(node1, Units::Pixels(100.0));
    world.set_cross(node1, Units::Pixels(150.0));
    world.set_main_after(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_main(node2, Units::Pixels(100.0));
    world.set_cross(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 150.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 100.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 150.0, width: 150.0, height: 100.0 })
    );
}

#[test]
fn percentage_main_after_pixels_main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_main(node1, Units::Pixels(100.0));
    world.set_cross(node1, Units::Pixels(150.0));
    world.set_main_after(node1, Units::Percentage(25.0));

    let node2 = world.add(Some(root));
    world.set_main(node2, Units::Pixels(100.0));
    world.set_cross(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 250.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 100.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 250.0, width: 150.0, height: 100.0 })
    );
}

#[test]
fn stretch_main_after_pixels_main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_main(node1, Units::Pixels(100.0));
    world.set_cross(node1, Units::Pixels(150.0));
    world.set_main_after(node1, Units::Stretch(1.0));

    let node2 = world.add(Some(root));
    world.set_main(node2, Units::Pixels(100.0));
    world.set_cross(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 100.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 500.0, width: 150.0, height: 100.0 })
    );
}

// #[test]
// fn pixels_main_before_percentage_main() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(600.0));
//     world.set_cross(root, Units::Pixels(600.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Percentage(25.0));
//     world.set_cross(node, Units::Pixels(150.0));
//     world.set_main_before(node, Units::Pixels(50.0));

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 50.0, posy: 0.0, width: 150.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 50.0, width: 150.0, height: 150.0 })
//     );
// }

// #[test]
// fn percentage_main_before_percentage_main() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(600.0));
//     world.set_cross(root, Units::Pixels(600.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Percentage(25.0));
//     world.set_cross(node, Units::Pixels(150.0));
//     world.set_main_before(node, Units::Percentage(50.0));

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 300.0, posy: 0.0, width: 150.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 300.0, width: 150.0, height: 150.0 })
//     );
// }

// #[test]
// fn stretch_main_before_percentage_main() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(600.0));
//     world.set_cross(root, Units::Pixels(600.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Percentage(25.0));
//     world.set_cross(node, Units::Pixels(150.0));
//     world.set_main_before(node, Units::Stretch(1.0));

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 450.0, posy: 0.0, width: 150.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 450.0, width: 150.0, height: 150.0 })
//     );
// }

// #[test]
// fn pixels_main_before_stretch_main() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(600.0));
//     world.set_cross(root, Units::Pixels(600.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Stretch(1.0));
//     world.set_cross(node, Units::Pixels(150.0));
//     world.set_main_before(node, Units::Pixels(50.0));

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 50.0, posy: 0.0, width: 550.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 50.0, width: 150.0, height: 550.0 })
//     );
// }

// #[test]
// fn percentage_main_before_stretch_main() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(600.0));
//     world.set_cross(root, Units::Pixels(600.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Stretch(1.0));
//     world.set_cross(node, Units::Pixels(150.0));
//     world.set_main_before(node, Units::Percentage(50.0));

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 300.0, width: 150.0, height: 300.0 })
//     );
// }

// #[test]
// fn stretch_main_before_stretch_main() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(600.0));
//     world.set_cross(root, Units::Pixels(600.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Stretch(1.0));
//     world.set_cross(node, Units::Pixels(150.0));
//     world.set_main_before(node, Units::Stretch(1.0));

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 300.0, posy: 0.0, width: 300.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 300.0, width: 150.0, height: 300.0 })
//     );
// }

// // Test to make sure single node with main_before isn't wrapping when container is too small
// #[test]
// fn pixels_main_before_pixels_main_no_wrap() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(300.0));
//     world.set_cross(root, Units::Pixels(300.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Pixels(400.0));
//     world.set_cross(node, Units::Pixels(150.0));
//     world.set_main_before(node, Units::Pixels(50.0));

//     let root_bc = BoxConstraints { min: (300.0, 300.0), max: (300.0, 300.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 50.0, posy: 0.0, width: 400.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (300.0, 300.0), max: (300.0, 300.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 50.0, width: 150.0, height: 400.0 })
//     );
// }

// #[test]
// fn percentage_main_before_pixels_main_no_wrap() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(300.0));
//     world.set_cross(root, Units::Pixels(300.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Pixels(400.0));
//     world.set_cross(node, Units::Pixels(150.0));
//     world.set_main_before(node, Units::Percentage(50.0));

//     let root_bc = BoxConstraints { min: (300.0, 300.0), max: (300.0, 300.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 150.0, posy: 0.0, width: 400.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (300.0, 300.0), max: (300.0, 300.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 150.0, width: 150.0, height: 400.0 })
//     );
// }

// #[test]
// fn stretch_main_before_pixels_main_no_wrap() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(300.0));
//     world.set_cross(root, Units::Pixels(300.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Pixels(400.0));
//     world.set_cross(node, Units::Pixels(150.0));
//     world.set_main_before(node, Units::Stretch(1.0));

//     let root_bc = BoxConstraints { min: (300.0, 300.0), max: (300.0, 300.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (300.0, 300.0), max: (300.0, 300.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 400.0 })
//     );
// }

// // Test of main_before on node which has wrapped
// #[test]
// fn pixels_main_before_wrap() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(300.0));
//     world.set_cross(root, Units::Pixels(300.0));

//     let node1 = world.add(Some(root));
//     world.set_main(node1, Units::Pixels(400.0));
//     world.set_cross(node1, Units::Pixels(150.0));

//     let node2 = world.add(Some(root));
//     world.set_main(node2, Units::Pixels(400.0));
//     world.set_cross(node2, Units::Pixels(150.0));
//     world.set_main_before(node2, Units::Pixels(50.0));

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node1),
//         Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 150.0 })
//     );

//     assert_eq!(
//         world.cache.bounds(node2),
//         Some(&Rect { posx: 50.0, posy: 150.0, width: 400.0, height: 150.0 })
//     );

//     world.set_layout_type(root, LayoutType::Column);

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node1),
//         Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 400.0 })
//     );

//     assert_eq!(
//         world.cache.bounds(node2),
//         Some(&Rect { posx: 150.0, posy: 50.0, width: 150.0, height: 400.0 })
//     );
// }
