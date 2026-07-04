use morphorm::*;
use morphorm_ecs::*;

#[test]
fn aspect_ratio_leaf_derives_height_from_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_alignment(root, Alignment::TopLeft);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(300.0));
    world.set_height(node, Units::Auto);
    world.set_aspect_ratio(node, 2.0);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 150.0 }));
}

#[test]
fn aspect_ratio_leaf_derives_width_from_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_alignment(root, Alignment::TopLeft);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Pixels(120.0));
    world.set_aspect_ratio(node, 1.5);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 180.0, height: 120.0 }));
}

#[test]
fn aspect_ratio_leaf_column_parent_axis_conversion() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Column);
    world.set_alignment(root, Alignment::TopLeft);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Pixels(300.0));
    world.set_aspect_ratio(node, 2.0);

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 600.0, height: 300.0 }));
}

#[test]
fn aspect_ratio_container_both_auto_with_children() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_alignment(root, Alignment::TopLeft);

    let container = world.add(Some(root));
    world.set_width(container, Units::Auto);
    world.set_height(container, Units::Auto);
    world.set_aspect_ratio(container, 1.0);

    let child = world.add(Some(container));
    world.set_width(child, Units::Pixels(200.0));
    world.set_height(child, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(container), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 100.0 }));
}

#[test]
fn aspect_ratio_min_width_can_override_ratio_result() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_alignment(root, Alignment::TopLeft);

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Pixels(100.0));
    world.set_aspect_ratio(node, 2.0);
    world.set_min_width(node, Units::Pixels(250.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 250.0, height: 100.0 }));
}

#[test]
fn aspect_ratio_max_height_can_override_ratio_result() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_alignment(root, Alignment::TopLeft);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(400.0));
    world.set_height(node, Units::Auto);
    world.set_aspect_ratio(node, 2.0);
    world.set_max_height(node, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 400.0, height: 100.0 }));
}
