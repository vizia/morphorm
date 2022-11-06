mod common;
use std::alloc::Layout;

use common::*;

fn main() {

        let mut world = World::default();
    
        let root = world.add(None);
        world.set_main(root, Units::Pixels(600.0));
        world.set_cross(root, Units::Pixels(600.0));
        world.set_child_main_before(root, Units::Pixels(50.0));
        world.set_child_main_after(root, Units::Pixels(50.0));
        // world.set_child_cross_before(root, Units::Pixels(50.0));
        // world.set_child_cross_after(root, Units::Pixels(50.0));
    
        let node1 = world.add(Some(root));
        world.set_main(node1, Units::Pixels(200.0));
        world.set_cross(node1, Units::Stretch(1.0));
    
        let node2 = world.add(Some(root));
        world.set_main(node2, Units::Pixels(200.0));
        world.set_cross(node2, Units::Stretch(1.0));
    
        let node3 = world.add(Some(root));
        world.set_main(node3, Units::Pixels(200.0));
        world.set_cross(node3, Units::Stretch(1.0));
    
        let root_bc = BoxConstraints { min: (600.0, 600.0), max: (600.0, 600.0) };
    
        layout(&root, LayoutType::Row, &root_bc, &mut world.cache, &world.tree, &world.store);
    
    render(world, root);
}
