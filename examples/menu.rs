mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Row);

    let node = world.add(Some(root));
    world.set_width(node, Units::Pixels(100.0));
    world.set_height(node, Units::Auto);
    world.set_layout_type(node, LayoutType::Row);

    let child1 = world.add(Some(node));
    world.set_width(child1, Units::Stretch(1.0));
    world.set_min_height(child1, Units::Auto);
    world.set_height(child1, Units::Stretch(1.0));
    world.set_layout_type(child1, LayoutType::Row);
    world.set_content_size(child1, |_, width, _height| {
        let width = width.unwrap();
        (width, 50.0)
    });

    let child2 = world.add(Some(node));
    world.set_width(child2, Units::Stretch(1.0));
    world.set_min_height(child2, Units::Auto);
    world.set_height(child2, Units::Stretch(1.0));
    world.set_layout_type(child2, LayoutType::Row);
    world.set_content_size(child2, |_, width, _height| {
        let width = width.unwrap();
        (width, 80.0)
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    render(world, root);
}
