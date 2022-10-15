use morphorm::*;
use morphorm_ecs::*;

#[test]
fn content_size() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(400.0));
    world.set_cross(root, Units::Pixels(400.0));

    let node = world.add(Some(root));
    world.set_main(node, Units::Pixels(400.0));
    world.set_cross(node, Units::Auto);
    world.set_content_size(node, |main| 100.0 * (400.0 / main));

    let root_bc = BoxConstraints { min: (200.0, 200.0), max: (200.0, 200.0) };

    layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.width(root), 400.0);
    assert_eq!(world.cache.height(root), 400.0);
    assert_eq!(world.cache.posx(root), 0.0);
    assert_eq!(world.cache.posy(root), 0.0);
    assert_eq!(world.cache.width(node), 400.0);
    assert_eq!(world.cache.height(node), 100.0);
    assert_eq!(world.cache.posx(node), 0.0);
    assert_eq!(world.cache.posy(node), 0.0);

    // world.set_layout_type(root, LayoutType::Column);

    // let root_bc = BoxConstraints { min: (200.0, 200.0), max: (200.0, 200.0) };

    // layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);

    // assert_eq!(world.cache.width(root), 200.0);
    // assert_eq!(world.cache.height(root), 200.0);
    // assert_eq!(world.cache.posx(root), 0.0);
    // assert_eq!(world.cache.posy(root), 0.0);
    // assert_eq!(world.cache.width(node), 150.0);
    // assert_eq!(world.cache.height(node), 100.0);
    // assert_eq!(world.cache.posx(node), 0.0);
    // assert_eq!(world.cache.posy(node), 0.0);
}
