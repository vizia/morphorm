use std::ops::Add;

use morphorm::Cache;
use morphorm::*;
use morphorm_ecs::*;

#[test]
fn absolute_layout_align_items_and_justify_content_flex_end() {
    let mut world = World::default();

    let root = world.add(None);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(110.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_child_top(node, Units::Stretch(1.0));
    world.set_child_left(node, Units::Stretch(1.0));

    let node0 = world.add(Some(node));
    world.set_width(node0, Units::Pixels(60.0));
    world.set_height(node0, Units::Pixels(40.0));
    world.set_position_type(node0, PositionType::SelfDirected);

    layout(&mut world.cache, &world.tree, &world.store);

    assert_eq!(world.cache.width(node), 110.0);
    assert_eq!(world.cache.height(node), 100.0);
    assert_eq!(world.cache.posx(node), 0.0);
    assert_eq!(world.cache.posy(node), 0.0);
    assert_eq!(world.cache.width(node0), 60.0);
    assert_eq!(world.cache.height(node0), 40.0);
    assert_eq!(world.cache.posx(node0), 50.0);
    assert_eq!(world.cache.posy(node0), 60.0);
}
