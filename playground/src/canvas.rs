use vizia::prelude::*;

use ecs::Store;
use morph::{Cache, Node};

use morphorm as morph;
use morphorm_ecs as ecs;
use vizia::vg;

use crate::{AppData, AppEvent};

pub struct CanvasView {}

impl CanvasView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {})
    }
}

impl View for CanvasView {
    fn element(&self) -> Option<&'static str> {
        Some("canvas")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if geo.contains(GeoChanged::WIDTH_CHANGED) || geo.contains(GeoChanged::HEIGHT_CHANGED) {
                    cx.emit(AppEvent::SetCanvasSize(
                        cx.cache.get_width(cx.current()) - 100.0,
                        cx.cache.get_height(cx.current()) - 100.0,
                    ));
                }
            }

            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                if let Some(app_data) = cx.data::<AppData>() {
                    let posx = cx.cache.get_posx(cx.current()) + 50.0;
                    let posy = cx.cache.get_posy(cx.current()) + 50.0;
                    let selected = select_node(
                        &app_data.root_node,
                        &app_data.world.tree,
                        &app_data.world.cache,
                        posx,
                        posy,
                        cx.mouse().cursorx,
                        cx.mouse().cursory,
                    );
                    // println!("selected: {:?}", selected);
                    if cx.modifiers().contains(Modifiers::SHIFT) {
                        if let Some(selected) = selected {
                            cx.emit(AppEvent::MultiSelectNode(*selected));
                        }
                    } else {
                        cx.emit(AppEvent::SelectNode(selected.copied()));
                    }
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut vizia::view::Canvas) {
        if let Some(app_data) = cx.data::<AppData>() {
            let bounds = cx.bounds();

            let mut path = vg::Path::new();
            path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
            let background_color = cx.background_color();
            canvas.fill_path(&path, &vg::Paint::color(background_color.into()));

            draw_node(
                &app_data.root_node,
                &app_data.selected_nodes,
                &app_data.world.tree,
                &app_data.world.cache,
                &app_data.world.store,
                (bounds.x + 50.0, bounds.y + 50.0),
                canvas,
            )

            // let root_node = app_data.root_node;

            // app_data.world.set_width(root_node, morph::Units::Pixels(bounds.w));
            // app_data.world.set_height(root_node, morph::Units::Pixels(bounds.h));

            // layout(&root_node, None, bounds.w, bounds.h, &mut app_data.world.cache, &app_data.world.tree, &app_data.world.store);
        }
    }
}

fn select_node<'a, N: Node<CacheKey = ecs::Entity>>(
    node: &'a N,
    tree: &'a N::Tree,
    cache: &impl Cache<Node = N>,
    parent_posx: f32,
    parent_posy: f32,
    mousex: f32,
    mousey: f32,
) -> Option<&'a N> {
    let posx = parent_posx + cache.posx(node);
    let posy = parent_posy + cache.posy(node);
    let width = cache.width(node);
    let height = cache.height(node);

    let mut selected_child = None;
    let children = node.children(tree).collect::<Vec<_>>();
    for child in children.into_iter().rev() {
        selected_child = select_node(child, tree, cache, posx, posy, mousex, mousey);
        if selected_child.is_some() {
            break;
        }
    }

    if selected_child.is_none() {
        if mousex >= posx && mousex < posx + width && mousey >= posy && mousey < posy + height {
            Some(node)
        } else {
            None
        }
    } else {
        selected_child
    }
}

fn draw_node<N: Node<CacheKey = ecs::Entity>>(
    node: &N,
    selected_nodes: &Option<Vec<N>>,
    tree: &N::Tree,
    cache: &impl Cache<Node = N>,
    store: &Store,
    parent_pos: (f32, f32),
    canvas: &mut Canvas,
) {
    let posx = cache.posx(node);
    let posy = cache.posy(node);
    let width = cache.width(node);
    let height = cache.height(node);

    let red = store.red.get(node.key()).unwrap_or(&0u8);
    let green = store.green.get(node.key()).unwrap_or(&0u8);
    let blue = store.blue.get(node.key()).unwrap_or(&0u8);

    let mut path = vg::Path::new();
    path.rect(parent_pos.0 + posx, parent_pos.1 + posy, width, height);
    let paint = vg::Paint::color(vg::Color::rgb(*red, *green, *blue));
    canvas.fill_path(&path, &paint);

    if let Some(selected_nodes) = selected_nodes {
        for selected_node in selected_nodes {
            if node.key() == selected_node.key() {
                let mut selection_paint = vg::Paint::color(vg::Color::rgb(72, 113, 174));
                selection_paint.set_line_width(4.0);
                canvas.stroke_path(&path, &selection_paint);
            }
        }
    }

    // if let Some(text) = store.text.get(&node.key()) {
    //     let mut paint = vg::Paint::color(vg::Color::black());
    //     paint.set_font_size(48.0);
    //     paint.set_text_align(vg::Align::Left);
    //     paint.set_text_baseline(vg::Baseline::Top);
    //     paint.set_font(&vec![font]);

    //     let font_metrics = canvas.measure_font(&paint).expect("Error measuring font");

    //     let mut y = 0.0;
    //     if let Ok(text_lines) = canvas.break_text_vec(width.ceil(), text, &paint) {
    //         //println!("font height: {}", font_metrics.height());
    //         for line in text_lines.into_iter() {
    //             let _ = canvas.fill_text(parent_posx + posx, parent_posy + posy + y, &text[line], &paint);
    //             // println!("{} {}", y, font_metrics.height());
    //             y += font_metrics.height();
    //         }
    //     }
    // } else {
    //     let mut paint = vg::Paint::color(vg::Color::black());
    //     paint.set_font_size(48.0);
    //     paint.set_text_align(vg::Align::Center);
    //     paint.set_text_baseline(vg::Baseline::Middle);
    //     paint.set_font(&vec![font]);

    //     let _ = canvas.fill_text(
    //         parent_posx + posx + width / 2.0,
    //         parent_posy + posy + height / 2.0,
    //         &node.key().0.to_string(),
    //         &paint,
    //     );
    // }

    for child in node.children(tree) {
        draw_node(child, selected_nodes, tree, cache, store, (posx + parent_pos.0, posy + parent_pos.1), canvas);
    }
}
