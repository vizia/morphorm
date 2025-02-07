use std::num::NonZeroU32;

use femtovg::FontId;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin::display::GetGlDisplay;
use glutin::prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor};
use glutin::surface::{GlSurface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use glutin_winit::DisplayBuilder;
pub use morphorm::*;
pub use morphorm_ecs::*;

use raw_window_handle::HasRawWindowHandle;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
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
    let events_loop = EventLoop::new();

    let (mut canvas, window, context, surface) = {
        let window_builder =
            WindowBuilder::new().with_inner_size(winit::dpi::PhysicalSize::new(600, 600)).with_title("Morphorm Demo");

        let template = ConfigTemplateBuilder::new().with_alpha_size(8);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(&events_loop, template, |configs| {
                // Find the config with the maximum number of samples, so our triangle will
                // be smooth.
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();

        let raw_window_handle = Some(window.raw_window_handle());

        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);
        let fallback_context_attributes =
            ContextAttributesBuilder::new().with_context_api(ContextApi::Gles(None)).build(raw_window_handle);
        let mut not_current_gl_context = Some(unsafe {
            gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                gl_display.create_context(&gl_config, &fallback_context_attributes).expect("failed to create context")
            })
        });

        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

        let gl_context = not_current_gl_context.take().unwrap().make_current(&surface).unwrap();

        surface.set_swap_interval(&gl_context, SwapInterval::DontWait).unwrap();

        let renderer = unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let size = window.inner_size();
        canvas.set_size(size.width, size.height, 1.0);
        canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(255, 80, 80));

        (canvas, window, gl_context, surface)
    };

    let font = canvas.add_font("examples/common/Roboto-Regular.ttf").expect("Failed to load font file");

    events_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(size) => {
                    surface.resize(&context, size.width.try_into().unwrap(), size.height.try_into().unwrap());

                    let layout_type = world.store.layout_type.get(root).cloned().unwrap_or_default();
                    match layout_type {
                        LayoutType::Row => {
                            world.set_width(root, Units::Pixels(size.width as f32));
                            world.set_height(root, Units::Pixels(size.height as f32));
                        }

                        LayoutType::Column => {
                            world.set_height(root, Units::Pixels(size.height as f32));
                            world.set_width(root, Units::Pixels(size.width as f32));
                        }

                        _ => {}
                    };

                    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());
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
                surface.swap_buffers(&context).unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
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

    let red = store.red.get(node.key()).unwrap_or(&0u8);
    let green = store.green.get(node.key()).unwrap_or(&0u8);
    let blue = store.blue.get(node.key()).unwrap_or(&0u8);

    let mut path = Path::new();
    path.rect(parent_posx + posx, parent_posy + posy, width, height);
    let paint = Paint::color(Color::rgb(*red, *green, *blue));
    canvas.fill_path(&mut path, &paint);

    if let Some(text) = store.text.get(node.key()) {
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
