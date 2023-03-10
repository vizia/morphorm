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
    world.set_text_wrap(node, TextWrap::None);
    world.set_content_size(node, move |store, width, height| content_size(node, store, width, height));

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
    world.set_text_wrap(node, TextWrap::Soft);
    world.set_content_size(node, move |store, width, height| content_size(node, store, width, height));

    let container3 = world.add(Some(container));
    world.set_width(container3, Units::Auto);
    world.set_height(container3, Units::Auto);
    world.set_child_space(container3, Units::Pixels(10.0));
    world.set_layout_type(container3, LayoutType::Row);
    world.set_col_between(container3, Units::Pixels(10.0));

    let node = world.add(Some(container3));
    world.set_width(node, Units::Pixels(150.0));
    world.set_height(node, Units::Auto);
    world.set_layout_type(node, LayoutType::Row);
    world.set_text(node, "This is some text");
    world.set_text_wrap(node, TextWrap::Soft);
    world.set_content_size(node, move |store, width, height| content_size(node, store, width, height));

    let container4 = world.add(Some(container));
    world.set_width(container4, Units::Auto);
    world.set_height(container4, Units::Auto);
    world.set_child_space(container4, Units::Pixels(10.0));
    world.set_layout_type(container4, LayoutType::Row);
    world.set_col_between(container4, Units::Pixels(10.0));

    let node = world.add(Some(container4));
    world.set_width(node, Units::Auto);
    world.set_height(node, Units::Auto);
    world.set_layout_type(node, LayoutType::Row);
    world.set_text(node, "This is\nsome text");
    world.set_text_wrap(node, TextWrap::Hard);
    world.set_content_size(node, move |store, width, height| content_size(node, store, width, height));

    root.layout(&mut world.cache, &world.tree, &world.store);

    render(world, root);
}

fn content_size(node: Entity, store: &Store, width: Option<f32>, height: Option<f32>) -> (f32, f32) {
    let text = store.text.get(&node).unwrap();
    let mut paint = femtovg::Paint::color(femtovg::Color::black());
    paint.set_font_size(48.0);
    paint.set_text_align(femtovg::Align::Left);
    paint.set_text_baseline(femtovg::Baseline::Top);
    paint.set_font(&vec![store.font_id.unwrap()]);
    // let should_wrap = store.text_wrap.get(&node).copied().unwrap_or_default();
    let text_wrap = store.text_wrap.get(&node).copied().unwrap_or_default();

    let max_width = if let Some(width) = width {
        width
    } else {
        match text_wrap {
            TextWrap::None | TextWrap::Hard => f32::MAX,
            TextWrap::Soft | TextWrap::All => {
                let mut max_word = 0.0f32;
                for word in text.unicode_words() {
                    if let Ok(text_metrics) = store.text_context.measure_text(0.0, 0.0, word, &paint) {
                        max_word = max_word.max(text_metrics.width());
                    }
                }
                max_word.ceil()
            }
        }
    };

    let font_metrics = store.text_context.measure_font(&paint).expect("Error measuring font");
    let (text_width, text_height) = if let Ok(text_lines) = store.text_context.break_text_vec(max_width, text, &paint) {
        let text_height = font_metrics.height() * text_lines.len() as f32;
        let mut text_width = 0.0f32;
        for line in text_lines {
            let line_text = &text[line];
            if let Ok(text_metrics) = store.text_context.measure_text(0.0, 0.0, line_text, &paint) {
                text_width = text_width.max(text_metrics.width());
            }
        }
        (text_width, text_height)
    } else {
        (0.0, 0.0)
    };

    let height = if let Some(height) = height { height } else { text_height };

    (text_width, height)
}
