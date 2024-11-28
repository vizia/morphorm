mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_child_space(root, Units::Stretch(1.0));

    let node = world.add(Some(root));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);

    let child = world.add(Some(node));
    world.set_width(child, Units::Stretch(1.0));
    world.set_min_width(child, Units::Auto);
    world.set_height(child, Units::Pixels(50.0));
    world.set_content_size(child, |_, _, height| {
        let h = height.unwrap();
        (50.0, h)
    });

    let child = world.add(Some(node));
    world.set_width(child, Units::Stretch(1.0));
    world.set_min_width(child, Units::Auto);
    world.set_height(child, Units::Pixels(50.0));
    world.set_content_size(child, |_, _, height| {
        let h = height.unwrap();
        (60.0, h)
    });

    let child = world.add(Some(node));
    world.set_width(child, Units::Stretch(1.0));
    world.set_min_width(child, Units::Auto);
    world.set_height(child, Units::Pixels(50.0));
    world.set_content_size(child, |_, _, height| {
        let h = height.unwrap();
        (130.0, h)
    });

    let child = world.add(Some(node));
    world.set_width(child, Units::Stretch(1.0));
    world.set_min_width(child, Units::Auto);
    world.set_height(child, Units::Pixels(50.0));
    world.set_content_size(child, |_, _, height| {
        let h = height.unwrap();
        (80.0, h)
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    render(world, root);
}
