use morphorm::*;
use morphorm_ecs::*;

// # Parent
// root: pixels(600.0) main, pixels(600.0) cross
// # Nodes
// node1: pixels(400.0) main, stretch(1.0) cross
// node2: pixels(400.0) main, stretch(1.0) cross
//
// Expected Result
// Parent Layout Type: Row
//     node1: px: 0.0, py: 0.0,   w: 400.0, h: 300.0
//     node2: px: 0.0, py: 300.0, w: 400.0, h: 300.0
// Parent Layout Type: Column
//     node1: px: 0.0,   py: 0.0, w: 300.0, h: 400.0
//     node2: px: 300.0, py: 0.0, w: 300.0, h: 400.0
#[test]
fn stretch_stretch_wrap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_main(node1, Units::Pixels(400.0));
    world.set_cross(node1, Units::Stretch(1.0));

    let node2 = world.add(Some(root));
    world.set_main(node2, Units::Pixels(400.0));
    world.set_cross(node2, Units::Stretch(1.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect{ posx: 0.0, posy: 0.0, width: 400.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect{ posx: 0.0, posy: 300.0, width: 400.0, height: 300.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect{ posx: 0.0, posy: 0.0, width: 300.0, height: 400.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect{ posx: 300.0, posy: 0.0, width: 300.0, height: 400.0 }));

}

// # Parent
// root: pixels(600.0) main, pixels(600.0) cross
// # Nodes
// node1: pixels(400.0) main, stretch(1.0) cross
// node2: pixels(400.0) main, pixels(200.0) cross
//
// Expected Result
// Parent Layout Type: Row
//     node1: px: 0.0, py: 0.0,   w: 400.0, h: 400.0
//     node2: px: 0.0, py: 400.0, w: 400.0, h: 200.0
// Parent Layout Type: Column
//     node1: px: 0.0,   py: 0.0, w: 400.0, h: 400.0
//     node2: px: 400.0, py: 0.0, w: 200.0, h: 400.0
#[test]
fn stretch_pixels_wrap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_main(node1, Units::Pixels(400.0));
    world.set_cross(node1, Units::Stretch(1.0));

    let node2 = world.add(Some(root));
    world.set_main(node2, Units::Pixels(400.0));
    world.set_cross(node2, Units::Pixels(200.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect{ posx: 0.0, posy: 0.0, width: 400.0, height: 400.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect{ posx: 0.0, posy: 400.0, width: 400.0, height: 200.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect{ posx: 0.0, posy: 0.0, width: 400.0, height: 400.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect{ posx: 400.0, posy: 0.0, width: 200.0, height: 400.0 }));
}


// # Parent
// root: pixels(600.0) main, pixels(600.0) cross
// # Nodes
// node1: pixels(400.0) main, pixels(200.0) cross
// node2: pixels(400.0) main, stretch(1.0) cross
//
// Expected Result
// Parent Layout Type: Row
//     node1: px: 0.0, py: 0.0,   w: 400.0, h: 200.0
//     node2: px: 0.0, py: 200.0, w: 400.0, h: 400.0
// Parent Layout Type: Column
//     node1: px: 0.0,   py: 0.0, w: 200.0, h: 400.0
//     node2: px: 200.0, py: 0.0, w: 400.0, h: 400.0
#[test]
fn pixels_stretch_wrap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_main(node1, Units::Pixels(400.0));
    world.set_cross(node1, Units::Pixels(200.0));

    let node2 = world.add(Some(root));
    world.set_main(node2, Units::Pixels(400.0));
    world.set_cross(node2, Units::Stretch(1.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect{ posx: 0.0, posy: 0.0, width: 400.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect{ posx: 0.0, posy: 200.0, width: 400.0, height: 400.0 }));

    world.set_layout_type(root, LayoutType::Column);

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.bounds(node1), Some(&Rect{ posx: 0.0, posy: 0.0, width: 200.0, height: 400.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect{ posx: 200.0, posy: 0.0, width: 400.0, height: 400.0 }));
}