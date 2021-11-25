mod common;
use common::*;

const STRING: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed enim nisi, lacinia eu quam vitae, faucibus gravida ligula. Nam in dapibus nisl. Nullam aliquam, diam et euismod posuere, eros nunc venenatis erat, nec eleifend leo sapien nec elit. Praesent quis suscipit nisi. Vivamus ac nisi tincidunt, maximus massa eget, venenatis velit. Pellentesque mauris eros, interdum maximus accumsan non, dictum ac neque. Aliquam condimentum dui eget tellus cursus, et ullamcorper odio egestas. Aenean fermentum mattis iaculis. Interdum et malesuada fames ac ante ipsum primis in faucibus. Mauris nec eleifend tellus, a laoreet dui. Sed consequat quis ex sed semper. Etiam varius condimentum tortor a pulvinar. Nullam aliquam mattis egestas. Curabitur semper est feugiat nibh consectetur, at cursus ipsum ultrices. Aenean lacinia gravida rutrum. Quisque sit amet lectus non odio bibendum vestibulum.";

fn main() {

    let mut world = World::default();

    let font_id = world.store.text_context.add_font_file("examples/common/Roboto-Regular.ttf").expect("Failed to load font file");
    world.store.font_id = Some(font_id);

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));

    world.set_layout_type(root, LayoutType::Column);
    world.set_child_space(root, Units::Stretch(1.0));

    // let parent = world.add(Some(root));
    // world.set_width(parent, Units::Stretch(1.0));
    // world.set_height(parent, Units::Stretch(1.0));
    // world.set_top(parent, Units::Pixels(0.0));
    // world.set_bottom(parent, Units::Pixels(0.0));
    // world.set_child_top(parent, Units::Stretch(1.0));
    // world.set_child_bottom(parent, Units::Stretch(1.0));


    let child1 = world.add(Some(root));
    world.set_width(child1, Units::Stretch(1.0));
    world.set_height(child1, Units::Stretch(1.0));
    world.set_text(child1, STRING.to_owned());
    world.set_intrinsic_size(child1, move |store, width| {
        if let Some(text) = store.text.get(&child1) {

            let mut paint = femtovg::Paint::color(femtovg::Color::black());
            paint.set_font_size(12.0);
            paint.set_text_align(femtovg::Align::Left);
            paint.set_text_baseline(femtovg::Baseline::Top);
            paint.set_font(&vec![store.font_id.unwrap()]);

            if let Ok(text_lines) = store.text_context.break_text_vec(width, text, paint) {
                if let Ok(font_metrics) = store.text_context.measure_font(paint) {
                    //println!("font height: {}", font_metrics.height());
                    return 20.0 * text_lines.len() as f32;
                }
            }
        }
        width
    });
    world.set_min_width(child1, Units::Pixels(300.0));

    let child2 = world.add(Some(root));
    world.set_width(child2, Units::Stretch(1.0));
    world.set_height(child2, Units::Stretch(1.0));

    let child3 = world.add(Some(root));
    world.set_width(child3, Units::Stretch(1.0));
    world.set_height(child3, Units::Stretch(1.0));


    layout(&mut world.cache, &world.tree, &world.store);


    render(world, root);
    
}