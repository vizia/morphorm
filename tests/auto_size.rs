use morphorm::*;
use morphorm_ecs::*;

#[test]
fn auto_main_pixels_child() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(600.0));
    world.set_cross(root, Units::Pixels(600.0));

    let node1 = world.add(Some(root));
    world.set_main(node1, Units::Auto);
    world.set_cross(node1, Units::Pixels(150.0));

    let node2 = world.add(Some(node1));
    world.set_main(node2, Units::Pixels(400.0));
    world.set_cross(node2, Units::Pixels(150.0));

    let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 150.0 })
    );

    world.set_layout_type(root, LayoutType::Column);

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(
        world.cache.bounds(node1),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 150.0, height: 150.0 })
    );

    assert_eq!(
        world.cache.bounds(node2),
        Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 150.0 })
    );
}
