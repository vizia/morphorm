mod common;
use common::*;

fn main() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_main(root, Units::Pixels(200.0));
    world.set_cross(root, Units::Pixels(200.0));
    world.set_layout_type(root, LayoutType::Column);
    
    //world.set_child_space(root, Units::Stretch(1.0));

    let node0 = world.add(Some(root));
    world.set_main(node0, Units::Auto);
    world.set_cross(node0, Units::Stretch(1.0));
    world.set_layout_type(node0, LayoutType::Column);

    let node1 = world.add(Some(node0));
    world.set_main(node1, Units::Auto);
    world.set_cross(node1, Units::Stretch(1.0));
    world.set_layout_type(node1, LayoutType::Column);

    let node2 = world.add(Some(node1));
    world.set_main(node2, Units::Auto);
    world.set_cross(node2, Units::Stretch(1.0));
    world.set_content_size(node2, |main| {
        100.0 * (400.0 / main)
    });

    let node00 = world.add(Some(root));
    world.set_main(node00, Units::Auto);
    world.set_cross(node00, Units::Stretch(1.0));
    world.set_layout_type(node00, LayoutType::Column);

    let node11 = world.add(Some(node00));
    world.set_main(node11, Units::Auto);
    world.set_cross(node11, Units::Stretch(1.0));
    world.set_layout_type(node11, LayoutType::Column);

    let node22 = world.add(Some(node11));
    world.set_main(node22, Units::Pixels(200.0));
    world.set_cross(node22, Units::Stretch(1.0));
    // world.set_content_size(node22, |main| {
    //     150.0 * (400.0 / main)
    // });

    let node000 = world.add(Some(root));
    world.set_main(node000, Units::Auto);
    world.set_cross(node000, Units::Stretch(1.0));
    world.set_layout_type(node000, LayoutType::Column);

    let node111 = world.add(Some(node000));
    world.set_main(node111, Units::Auto);
    world.set_cross(node111, Units::Stretch(1.0));
    world.set_layout_type(node111, LayoutType::Column);

    let node222 = world.add(Some(node111));
    world.set_main(node222, Units::Auto);
    world.set_cross(node222, Units::Stretch(1.0));
    world.set_content_size(node222, |main| {
        200.0 * (400.0 / main)
    });

    // let node0000 = world.add(Some(root));
    // world.set_main(node0000, Units::Auto);
    // world.set_cross(node0000, Units::Stretch(1.0));
    // world.set_layout_type(node0000, LayoutType::Column);

    // let node1111 = world.add(Some(node0000));
    // world.set_main(node1111, Units::Auto);
    // world.set_cross(node1111, Units::Stretch(1.0));
    // world.set_layout_type(node1111, LayoutType::Column);

    // let node2222 = world.add(Some(node1111));
    // world.set_main(node2222, Units::Auto);
    // world.set_cross(node2222, Units::Stretch(1.0));
    // world.set_content_size(node2222, |main| {
    //     250.0 * (400.0 / main)
    // });

    // let node0 = world.add(Some(root));
    // world.set_main(node0, Units::Auto);
    // world.set_cross(node0, Units::Stretch(1.0));
    // world.set_content_size(node0, |main| {
    //     100.0 * (400.0 / main)
    // });

    // let node0 = world.add(Some(root));
    // world.set_main(node0, Units::Auto);
    // world.set_cross(node0, Units::Stretch(1.0));
    // world.set_content_size(node0, |main| {
    //     100.0 * (400.0 / main)
    // });

    // let node0 = world.add(Some(root));
    // world.set_main(node0, Units::Auto);
    // world.set_cross(node0, Units::Stretch(1.0));
    // world.set_content_size(node0, |main| {
    //     100.0 * (400.0 / main)
    // });



    // world.set_layout_type(node0, LayoutType::Row);

    // let node1 = world.add(Some(root));
    // world.set_main(node1, Units::Pixels(100.0));
    // world.set_cross(node1, Units::Pixels(100.0));
    // world.set_layout_type(root, LayoutType::Row);


    // let node1 = world.add(Some(node0));
    // world.set_main(node1, Units::Stretch(1.0));
    // world.set_cross(node1, Units::Auto);

    // let node2 = world.add(Some(node1));
    // world.set_main(node2, Units::Stretch(1.0));
    // world.set_cross(node2, Units::Auto);

    // world.set_content_size(node2, |main| {
    //     (main / 4.0).round()
    // });

    // let node3 = world.add(Some(root));
    // world.set_main(node3, Units::Stretch(1.0));
    // world.set_cross(node3, Units::Pixels(100.0));

    // let node0 = world.add(Some(root));
    // world.set_width(node0, Units::Pixels(100.0));
    // world.set_height(node0, Units::Pixels(100.0));

    // let node0 = world.add(Some(root));
    // world.set_width(node0, Units::Pixels(100.0));
    // world.set_height(node0, Units::Pixels(100.0));

    // let node1 = world.add(Some(node0));
    // world.set_width(node1, Units::Stretch(1.0));
    // world.set_height(node1, Units::Stretch(1.0));
    // world.set_layout_type(node1, LayoutType::Row);
    // world.set_content_size(node1, |cross_size|{
    //     40000.0 / cross_size
    // });

    // let node2 = world.add(Some(node0));
    // world.set_width(node2, Units::Pixels(50.0));
    // world.set_height(node2, Units::Pixels(40.0));
    // world.set_child_space(parent, Units::Pixels(10.0));

    // world.set_layout_type(parent, LayoutType::Row);

    // let child1 = world.add(Some(parent));
    // world.set_width(child1, Units::Stretch(1.0));
    // world.set_height(child1, Units::Pixels(100.0));
    // world.set_min_width(child1, Units::Pixels(100.0));

    // let child2 = world.add(Some(parent));
    // world.set_width(child2, Units::Stretch(1.0));
    // world.set_height(child2, Units::Pixels(100.0));
    // world.set_min_width(child2, Units::Pixels(50.0));

    // let child3 = world.add(Some(parent));
    // world.set_width(child3, Units::Stretch(1.0));
    // world.set_height(child3, Units::Pixels(100.0));

    let root_bc = BoxConstraints { min: (200.0, 200.0), max: (200.0, 200.0) };

    layout(&root, LayoutType::Column, &root_bc, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}
