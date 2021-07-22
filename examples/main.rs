mod common;
use common::*;

fn main() {


    let test1 = Units2::pixels(100.0);
    println!("Test1: {:?}", test1);

    let test2 = Units2::pixels(100.0).min(50.0);
    println!("Test2: {:?}", test2);

    let test3 = Units2::pixels(100.0).min(50.0).max(150.0);
    println!("Test3: {:?}", test3);

    // let mut world = World::default();

    // let root = world.add(None);
    // world.set_width(root, Units::Pixels(1000.0));
    // world.set_height(root, Units::Pixels(600.0));
    // world.set_left(root, Units::Pixels(0.0));
    // world.set_top(root, Units::Pixels(0.0));

    // let child = world.add(Some(root));
    // world.set_width(child, Units::Percentage(50.0));
    // world.set_height(child, Units::Stretch(1.0));

    // // let _child2 = world.add(Some(root));
    // // world.set_width(child, Units::Stretch(1.0));
    // // world.set_height(child, Units::Stretch(1.0));
    // // world.set_left(child, Units::Pixels(50.0));


    // layout(&mut world.cache, &world.tree, &world.store);

    // let computed_width = world.cache.width(&child);
    // let computed_height = world.cache.height(&child);
    // let computed_posx = world.cache.posx(&child);
    // let computed_posy = world.cache.posy(&child);

    // println!("Computed Width: {}", computed_width);
    // println!("Computed Height: {}", computed_height);
    // println!("Computed PosX: {}", computed_posx);
    // println!("Computed PosY: {}", computed_posy);

    // render(world, root);
    
}
