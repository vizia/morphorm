mod common;
use common::*;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut world = World::default();

    let font_id =
        world.store.text_context.add_font_file("examples/common/Roboto-Regular.ttf").expect("Failed to load font file");
    world.store.font_id = Some(font_id);

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));

    let container = world.add(Some(root));
    world.set_width(container, Units::Auto);
    world.set_height(container, Units::Auto);
    world.set_child_space(container, Units::Pixels(10.0));
    world.set_layout_type(container, LayoutType::Row);
    world.set_col_between(container, Units::Pixels(10.0));

    let container1 = world.add(Some(container));
    world.set_width(container1, Units::Auto);
    world.set_height(container1, Units::Auto);
    world.set_child_space(container1, Units::Pixels(10.0));
    world.set_layout_type(container1, LayoutType::Row);
    world.set_col_between(container1, Units::Pixels(10.0));

    let node = world.add(Some(container1));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);
    world.set_layout_type(node, LayoutType::Row);
    world.set_text(node, "This is some text");
    world.set_text_wrap(node, false);
    world.set_content_main(node, move |store, _| content_width(node, store));
    world.set_content_cross(node, move |store, width| content_height(node, store, width));

    let container2 = world.add(Some(container));
    world.set_width(container2, Units::Auto);
    world.set_height(container2, Units::Auto);
    world.set_child_space(container2, Units::Pixels(10.0));
    world.set_layout_type(container2, LayoutType::Row);
    world.set_col_between(container2, Units::Pixels(10.0));

    let node = world.add(Some(container2));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);
    world.set_layout_type(node, LayoutType::Row);
    world.set_text(node, "This is some text");
    world.set_text_wrap(node, true);
    world.set_content_main(node, move |store, _| content_width(node, store));
    world.set_content_cross(node, move |store, width| content_height(node, store, width));

    layout(&root, None, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);

    render(world, root);
}

fn content_width(node: Entity, store: &Store) -> f32 {
    let text = store.text.get(&node).unwrap();
    let mut paint = femtovg::Paint::color(femtovg::Color::black());
    paint.set_font_size(48.0);
    paint.set_text_align(femtovg::Align::Left);
    paint.set_text_baseline(femtovg::Baseline::Top);
    paint.set_font(&vec![store.font_id.unwrap()]);
    let should_wrap = store.text_wrap.get(&node).copied().unwrap_or_default();

    // Figure out width of longest word
    let mut max_word = 0.0f32;
    for word in text.unicode_words() {
        if let Ok(text_metrics) = store.text_context.measure_text(0.0, 0.0, word, &paint) {
            max_word = max_word.max(text_metrics.width());
        }
    }

    let max_width = if should_wrap { max_word.ceil() } else { f32::MAX };

    if let Ok(text_lines) = store.text_context.break_text_vec(max_width, text, &paint) {
        let mut max_width = 0.0f32;
        for line in text_lines {
            let line_text = &text[line];
            if let Ok(text_metrics) = store.text_context.measure_text(0.0, 0.0, line_text, &paint) {
                max_width = max_width.max(text_metrics.width());
            }
        }
        return max_width;
    }

    0.0
}

fn content_height(node: Entity, store: &Store, width: f32) -> f32 {
    let text = store.text.get(&node).unwrap();
    let mut paint = femtovg::Paint::color(femtovg::Color::black());
    paint.set_font_size(48.0);
    paint.set_text_align(femtovg::Align::Center);
    paint.set_text_baseline(femtovg::Baseline::Middle);
    paint.set_font(&vec![store.font_id.unwrap()]);

    let should_wrap = store.text_wrap.get(&node).copied().unwrap_or_default();

    let max_width = if should_wrap { width } else { f32::MAX };

    let font_metrics = store.text_context.measure_font(&paint).expect("Error measuring font");
    if let Ok(text_lines) = store.text_context.break_text_vec(max_width, text, &paint) {
        let height = font_metrics.height() * text_lines.len() as f32;
        return height;
    }

    0.0
}
