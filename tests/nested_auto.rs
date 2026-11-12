use std::{cell::Cell, rc::Rc};

use morphorm::{LayoutType, Node, Units};
use morphorm_ecs::{Rect, World};

#[test]
fn percentage_sizes_and_limits_use_resolved_auto_parent_size() {
    for layout_type in [LayoutType::Row, LayoutType::Column] {
        for (size, minimum, maximum) in [
            (Units::Percentage(50.0), Units::Pixels(0.0), Units::Pixels(f32::MAX)),
            (Units::Pixels(16.0), Units::Percentage(50.0), Units::Pixels(f32::MAX)),
            (Units::Pixels(400.0), Units::Pixels(0.0), Units::Percentage(50.0)),
        ] {
            let mut world = World::default();
            let root = world.add(None);
            world.set_width(root, Units::Pixels(1000.0));
            world.set_height(root, Units::Pixels(1000.0));

            let parent = world.add(Some(root));
            world.set_width(parent, Units::Auto);
            world.set_height(parent, Units::Auto);
            world.set_layout_type(parent, layout_type);

            let sibling = world.add(Some(parent));
            world.set_width(sibling, Units::Pixels(200.0));
            world.set_height(sibling, Units::Pixels(200.0));

            let container = world.add(Some(parent));
            if layout_type == LayoutType::Row {
                world.set_width(container, Units::Pixels(10.0));
                world.set_height(container, size);
                world.set_min_height(container, minimum);
                world.set_max_height(container, maximum);
            } else {
                world.set_width(container, size);
                world.set_min_width(container, minimum);
                world.set_max_width(container, maximum);
                world.set_height(container, Units::Pixels(10.0));
            }

            let leaf = world.add(Some(container));
            world.set_width(leaf, Units::Percentage(100.0));
            world.set_height(leaf, Units::Percentage(100.0));

            root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

            let expected = if layout_type == LayoutType::Row { (10.0, 100.0) } else { (100.0, 10.0) };
            for node in [container, leaf] {
                let bounds = world.cache.bounds(node).unwrap();
                assert_eq!(
                    (bounds.width, bounds.height),
                    expected,
                    "{layout_type:?}, {size:?}, {minimum:?}, {maximum:?}"
                );
            }
        }
    }
}

#[test]
fn fixed_container_reuses_measurement_of_percentage_descendant() {
    let mut world = World::default();
    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(1000.0));

    let parent = world.add(Some(root));
    world.set_width(parent, Units::Auto);
    world.set_height(parent, Units::Auto);

    let container = world.add(Some(parent));
    world.set_width(container, Units::Pixels(100.0));
    world.set_height(container, Units::Pixels(80.0));

    let measurements = Rc::new(Cell::new(0));
    let leaf = world.add(Some(container));
    world.set_width(leaf, Units::Percentage(100.0));
    world.set_height(leaf, Units::Auto);
    world.set_content_size(leaf, {
        let measurements = measurements.clone();
        move |_, width, _| {
            measurements.set(measurements.get() + 1);
            (width.unwrap(), 16.0)
        }
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(measurements.get(), 1);
    assert_eq!(world.cache.bounds(parent), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 80.0 }));
    assert_eq!(world.cache.bounds(leaf), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 16.0 }));
}

#[test]
fn nested_auto_containers_measure_content_once() {
    for root_layout in [LayoutType::Row, LayoutType::Column] {
        let mut world = World::default();
        let root = world.add(None);
        world.set_width(root, Units::Pixels(1000.0));
        world.set_height(root, Units::Pixels(1000.0));
        world.set_layout_type(root, root_layout);

        let mut parent = root;
        let mut containers = Vec::new();
        for depth in 0..12 {
            let node = world.add(Some(parent));
            world.set_width(node, Units::Auto);
            world.set_height(node, Units::Auto);
            world.set_layout_type(node, if depth % 2 == 0 { LayoutType::Row } else { LayoutType::Column });
            containers.push(node);
            parent = node;
        }

        let measurements = Rc::new(Cell::new(0));
        let height = Rc::new(Cell::new(16.0));
        let leaf = world.add(Some(parent));
        world.set_width(leaf, Units::Auto);
        world.set_height(leaf, Units::Auto);
        world.set_content_size(leaf, {
            let measurements = measurements.clone();
            let height = height.clone();
            move |_, _, _| {
                measurements.set(measurements.get() + 1);
                (16.0, height.get())
            }
        });

        root.layout(&mut world.cache, &world.tree, &world.store, &mut ());
        assert_eq!(measurements.get(), 1, "initial layout must not repeatedly measure the same content");
        for node in containers.iter().copied().chain([leaf]) {
            assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 16.0, height: 16.0 }));
        }

        height.set(24.0);
        measurements.set(0);
        leaf.layout(&mut world.cache, &world.tree, &world.store, &mut ());
        assert_eq!(measurements.get(), 1, "incremental layout must not repeatedly measure the same content");
        for node in containers.iter().copied().chain([leaf]) {
            assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 0.0, posy: 0.0, width: 16.0, height: 24.0 }));
        }
    }
}
