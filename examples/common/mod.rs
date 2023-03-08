use femtovg::FontId;
use glutin::event::{ElementState, VirtualKeyCode};
pub use morphorm::*;
pub use morphorm_ecs::*;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use femtovg::{
    //CompositeOperation,
    renderer::OpenGl,
    Align,
    Baseline,
    Canvas,
    Color,
    Paint,
    Path,
};

pub fn render(mut world: World, root: Entity) {
    let event_loop = EventLoop::new();

    let (renderer, windowed_context) = {
        use glutin::ContextBuilder;

        let window_builder =
            WindowBuilder::new().with_inner_size(winit::dpi::PhysicalSize::new(1000, 600)).with_title("Morphorm Demo");

        let windowed_context =
            ContextBuilder::new().with_vsync(false).build_windowed(window_builder, &event_loop).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let renderer = unsafe {
            OpenGl::new_from_function(|s| windowed_context.get_proc_address(s) as *const _)
                .expect("Cannot create renderer")
        };

        (renderer, windowed_context)
    };

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

    let font = canvas.add_font("examples/common/Roboto-Regular.ttf").expect("Failed to load font file");

    event_loop.run(move |event, _, control_flow| {
        #[cfg(not(target_arch = "wasm32"))]
        let window = windowed_context.window();

        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                    let layout_type = world.store.layout_type.get(&root).cloned().unwrap_or_default();
                    match layout_type {
                        LayoutType::Row => {
                            world.set_width(root, Units::Pixels(physical_size.width as f32));
                            world.set_height(root, Units::Pixels(physical_size.height as f32));
                        }

                        LayoutType::Column => {
                            world.set_height(root, Units::Pixels(physical_size.height as f32));
                            world.set_width(root, Units::Pixels(physical_size.width as f32));
                        }
                    };

                    layout(&root, None, 600.0, 600.0, &mut world.cache, &world.tree, &world.store);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                    if input.virtual_keycode == Some(VirtualKeyCode::H) && input.state == ElementState::Pressed {
                        print_node(&root, &world.cache, &world.tree, true, false, String::new());
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let size = window.inner_size();

                canvas.set_size(size.width as u32, size.height as u32, 1.0);
                canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgbf(0.3, 0.3, 0.32));

                draw_node(&root, &world.tree, &world.cache, &world.store, 0.0, 0.0, font, &mut canvas);

                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => window.request_redraw(),
            _ => (),
        }
    });
}

fn draw_node<N: Node<CacheKey = Entity>>(
    node: &N,
    tree: &N::Tree,
    cache: &impl Cache<Node = N>,
    store: &Store,
    parent_posx: f32,
    parent_posy: f32,
    font: FontId,
    canvas: &mut Canvas<OpenGl>,
) {
    let posx = cache.posx(node);
    let posy = cache.posy(node);
    let width = cache.width(node);
    let height = cache.height(node);

    let red = store.red.get(&node.key()).unwrap_or(&0u8);
    let green = store.green.get(&node.key()).unwrap_or(&0u8);
    let blue = store.blue.get(&node.key()).unwrap_or(&0u8);

    let mut path = Path::new();
    path.rect(parent_posx + posx, parent_posy + posy, width, height);
    let paint = Paint::color(Color::rgb(*red, *green, *blue));
    canvas.fill_path(&mut path, &paint);

    if let Some(text) = store.text.get(&node.key()) {
        let mut paint = Paint::color(Color::black());
        paint.set_font_size(48.0);
        paint.set_text_align(Align::Left);
        paint.set_text_baseline(Baseline::Top);
        paint.set_font(&vec![font]);

        let font_metrics = canvas.measure_font(&paint).expect("Error measuring font");

        let mut y = 0.0;
        if let Ok(text_lines) = canvas.break_text_vec(width.ceil(), text, &paint) {
            //println!("font height: {}", font_metrics.height());
            for line in text_lines.into_iter() {
                let _ = canvas.fill_text(parent_posx + posx, parent_posy + posy + y, &text[line], &paint);
                // println!("{} {}", y, font_metrics.height());
                y += font_metrics.height();
            }
        }
    } else {
        let mut paint = Paint::color(Color::black());
        paint.set_font_size(48.0);
        paint.set_text_align(Align::Center);
        paint.set_text_baseline(Baseline::Middle);
        paint.set_font(&vec![font]);

        let _ = canvas.fill_text(
            parent_posx + posx + width / 2.0,
            parent_posy + posy + height / 2.0,
            &node.key().0.to_string(),
            &paint,
        );
    }

    for child in node.children(tree) {
        draw_node(child, tree, cache, store, posx + parent_posx, posy + parent_posy, font, canvas);
    }
}
