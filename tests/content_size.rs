// use morphorm::*;
// use morphorm_ecs::*;

// #[test]
// fn content_size() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(400.0));
//     world.set_cross(root, Units::Pixels(400.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Pixels(400.0));
//     world.set_cross(node, Units::Auto);
//     world.set_content_size(node, |main| 100.0 * (400.0 / main));

//     let root_bc = BoxConstraints { min: (400.0, 400.0), max: (400.0, 400.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(world.cache.width(root), 400.0);
//     assert_eq!(world.cache.height(root), 400.0);
//     assert_eq!(world.cache.posx(root), 0.0);
//     assert_eq!(world.cache.posy(root), 0.0);
//     assert_eq!(world.cache.width(node), 400.0);
//     assert_eq!(world.cache.height(node), 100.0);
//     assert_eq!(world.cache.posx(node), 0.0);
//     assert_eq!(world.cache.posy(node), 0.0);

//     // world.set_layout_type(root, LayoutType::Column);

//     // let root_bc = BoxConstraints { min: (200.0, 200.0), max: (200.0, 200.0) };

//     // layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     // assert_eq!(world.cache.width(root), 200.0);
//     // assert_eq!(world.cache.height(root), 200.0);
//     // assert_eq!(world.cache.posx(root), 0.0);
//     // assert_eq!(world.cache.posy(root), 0.0);
//     // assert_eq!(world.cache.width(node), 150.0);
//     // assert_eq!(world.cache.height(node), 100.0);
//     // assert_eq!(world.cache.posx(node), 0.0);
//     // assert_eq!(world.cache.posy(node), 0.0);
// }

// #[test]
// fn auto_main_pixels_child() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(600.0));
//     world.set_cross(root, Units::Pixels(600.0));

//     let node1 = world.add(Some(root));
//     world.set_cross(node1, Units::Auto);
//     world.set_main(node1, Units::Stretch(1.0));
//     world.set_child_space(node1, Units::Pixels(50.0));

//     let node2 = world.add(Some(node1));
//     world.set_main(node2, Units::Stretch(1.0));
//     world.set_cross(node2, Units::Stretch(1.0));

//     let node3 = world.add(Some(node1));
//     world.set_main(node3, Units::Stretch(1.0));
//     world.set_cross(node3, Units::Auto);
//     world.set_content_size(node3, |main| main);

//     let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(node1),
//         Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 350.0 })
//     );

//     assert_eq!(
//         world.cache.bounds(node2),
//         Some(&Rect { posx: 50.0, posy: 50.0, width: 250.0, height: 250.0 })
//     );

//     assert_eq!(
//         world.cache.bounds(node3),
//         Some(&Rect { posx: 300.0, posy: 50.0, width: 250.0, height: 250.0 })
//     );

//     // world.set_layout_type(root, LayoutType::Column);

//     // layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

//     // assert_eq!(
//     //     world.cache.bounds(node1),
//     //     Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 150.0 })
//     // );

//     // assert_eq!(
//     //     world.cache.bounds(node2),
//     //     Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 150.0 })
//     // );
// }
