use morphorm::*;
use morphorm_ecs::*;

#[test]
fn visibility() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_visibility(node, false);

    let child = world.add(Some(node));
    world.set_width(child, Units::Pixels(100.0));
    world.set_height(child, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 0.0, height: 0.0 }));
}
