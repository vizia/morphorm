mod common;

use common::*;

fn build_shallow_tree(world: &mut World, parent: Option<Entity>, depth: usize) {
    if depth > 0 {
        let node = world.add(parent);
        world.set_all_stretch(node);
        for _ in 0..10 {
            build_shallow_tree(world, Some(node), depth - 1)
        }
    }
}

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Row);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Stretch(1.0));
    world.set_layout_type(node1, LayoutType::Row);

    let node2 = world.add(Some(node1));
    world.set_width(node2, Units::Stretch(1.0));
    world.set_height(node2, Units::Stretch(1.0));
    // world.set_content_size(node2, |height| height);
    world.set_layout_type(node2, LayoutType::Row);

    layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);


    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));

    // world.set_layout_type(root, LayoutType::Row);

    // let node = world.add(Some(root));
    // world.set_width(node, Units::Pixels(100.0));
    // world.set_height(node, Units::Pixels(150.0));
    // world.set_left(node, Units::Stretch(1.0));

    // layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    // assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));

    // world.set_layout_type(root, LayoutType::Column);

    // layout(&root, LayoutType::Column, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    // assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 500.0, posy: 0.0, width: 100.0, height: 150.0 }));


    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));

    // world.set_layout_type(root, LayoutType::Row);

    // let node = world.add(Some(root));
    // world.set_width(node, Units::Stretch(1.0));
    // world.set_height(node, Units::Percentage(25.0));

    // // layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    // // assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 150.0 }));

    // world.set_layout_type(root, LayoutType::Column);

    // layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    // // assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 150.0 }));


    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));

    // world.set_layout_type(root, LayoutType::Row);

    // let node1 = world.add(Some(root));
    // world.set_width(node1, Units::Auto);
    // world.set_height(node1, Units::Pixels(150.0));
    // // world.set_layout_type(node1, LayoutType::Row);

    // let node2 = world.add(Some(node1));
    // world.set_width(node2, Units::Pixels(100.0));
    // world.set_height(node2, Units::Pixels(100.0));
    // world.set_right(node2, Units::Percentage(50.0));

    // layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    // // assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    // // world.set_layout_type(root, LayoutType::Column);

    // // layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    // // assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));

    // world.set_layout_type(root, LayoutType::Row);

    // let node1 = world.add(Some(root));
    // world.set_width(node1, Units::Auto);
    // world.set_height(node1, Units::Pixels(150.0));

    // let node2 = world.add(Some(node1));
    // world.set_width(node2, Units::Pixels(100.0));
    // world.set_height(node2, Units::Pixels(100.0));
    // world.set_left(node2, Units::Percentage(50.0));

    // layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    // // assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));

    // // world.set_layout_type(root, LayoutType::Column);

    // // layout(&root, LayoutType::Column, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    // // assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));


    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(600.0));
    // world.set_height(root, Units::Pixels(600.0));

    // world.set_layout_type(root, LayoutType::Row);

    // let node = world.add(Some(root));
    // world.set_width(node, Units::Pixels(100.0));
    // world.set_height(node, Units::Pixels(100.0));
    // world.set_min_height(node, Units::Pixels(200.0));

    // layout(&root, LayoutType::Row, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);


    render(world, root);
}
