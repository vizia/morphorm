// use morphorm::*;
// use morphorm_ecs::*;

// #[test]
// fn stretch_main_stretch_cross() {
//     let mut world = World::default();

//     let root = world.add(None);
//     world.set_main(root, Units::Pixels(200.0));
//     world.set_cross(root, Units::Pixels(200.0));

//     let node = world.add(Some(root));
//     world.set_main(node, Units::Stretch(1.0));
//     world.set_cross(node, Units::Stretch(1.0));

//     let root_bc = BoxConstraints { min: (200.0, 200.0), max: (200.0, 200.0) };

//     layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     assert_eq!(
//         world.cache.bounds(root),
//         Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 })
//     );

//     assert_eq!(
//         world.cache.bounds(node),
//         Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 })
//     );

//     // world.set_layout_type(root, LayoutType::Column);

//     // let root_bc = BoxConstraints { min: (200.0, 200.0), max: (200.0, 200.0) };

//     // layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

//     // assert_eq!(
//     //     world.cache.bounds(root),
//     //     Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 })
//     // );

//     // assert_eq!(
//     //     world.cache.bounds(node),
//     //     Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 })
//     // );
// }
