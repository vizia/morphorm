use femtovg::FontId;
use glutin::event::VirtualKeyCode;
use morphorm_ecs::tree::Tree;
pub use morphorm_ecs::{Entity, World};

pub use morphorm::*;

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
    let el = EventLoop::new();

    let (renderer, windowed_context) = {
        use glutin::ContextBuilder;

        let wb = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(1000, 600))
            .with_title("Morphorm Demo");

        let windowed_context =
            ContextBuilder::new().with_vsync(false).build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let renderer = OpenGl::new(|s| windowed_context.get_proc_address(s) as *const _)
            .expect("Cannot create renderer");

        (renderer, windowed_context)
    };

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

    let font =
        canvas.add_font("examples/common/Roboto-Regular.ttf").expect("Failed to load font file");

    //world.cache.set_width(root, 1000.0);
    //world.cache.set_height(root, 600.0);

    el.run(move |event, _, control_flow| {
        #[cfg(not(target_arch = "wasm32"))]
        let window = windowed_context.window();

        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                    let layout_type =
                        world.store.layout_type.get(&root).cloned().unwrap_or_default();
                    let mut root_bc = BoxConstraints::default();
                    match layout_type {
                        LayoutType::Row => {
                            world.set_main(root, Units::Pixels(physical_size.width as f32));
                            world.set_cross(root, Units::Pixels(physical_size.height as f32));

                            root_bc = BoxConstraints {
                                min: (physical_size.width as f32, physical_size.height as f32),
                                max: (physical_size.width as f32, physical_size.height as f32),
                            };
                        }

                        LayoutType::Column => {
                            world.set_main(root, Units::Pixels(physical_size.height as f32));
                            world.set_cross(root, Units::Pixels(physical_size.width as f32));

                            root_bc = BoxConstraints {
                                min: (physical_size.height as f32, physical_size.width as f32),
                                max: (physical_size.height as f32, physical_size.width as f32),
                            };
                        }

                        _ => {}
                    }

                    layout(
                        &root,
                        layout_type,
                        &root_bc,
                        &mut world.cache,
                        &world.tree,
                        &world.store,
                    );
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                    if input.virtual_keycode == Some(VirtualKeyCode::H) {
                        let nodes = world.tree.flatten();
                        for node in nodes.into_iter() {
                            println!(
                                "{:?} px: {:?} py: {:?} w: {:?} h: {:?}",
                                node,
                                world.cache.posx(node),
                                world.cache.posy(node),
                                world.cache.width(node),
                                world.cache.height(node)
                            );
                        }
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(
                    0,
                    0,
                    size.width as u32,
                    size.height as u32,
                    Color::rgbf(0.3, 0.3, 0.32),
                );

                draw_node(&root, &world, 0.0, 0.0, font, &mut canvas);

                // let mut global_posx = 0.0;
                // let mut global_posy = 0.0;
                // for node in world.tree.down_iter() {
                //     let posx = world.cache.posx(node);
                //     let posy = world.cache.posy(node);
                //     let width = world.cache.width(node);
                //     let height = world.cache.height(node);

                //     let red = world.store.red.get(&node).unwrap_or(&0u8);
                //     let green = world.store.green.get(&node).unwrap_or(&0u8);
                //     let blue = world.store.blue.get(&node).unwrap_or(&0u8);

                //     let mut path = Path::new();
                //     path.rect(global_posx + posx, global_posy + posy, width, height);
                //     let paint = Paint::color(Color::rgb(*red, *green, *blue));
                //     canvas.fill_path(&mut path, paint);

                //     let mut paint = Paint::color(Color::black());
                //     paint.set_font_size(24.0);
                //     paint.set_text_align(Align::Center);
                //     paint.set_text_baseline(Baseline::Middle);
                //     paint.set_font(&vec![font]);
                //     let _ = canvas.fill_text(
                //         global_posx + posx + width / 2.0,
                //         global_posy + posy + height / 2.0,
                //         &node.0.to_string(),
                //         paint,
                //     );

                //     global_posx += posx;
                //     global_posy += posy;
                // }

                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => window.request_redraw(),
            _ => (),
        }
    });
}

fn draw_node<N: Node<Tree = Tree, CacheKey = Entity>>(
    node: &N,
    world: &World,
    parent_posx: f32,
    parent_posy: f32,
    font: FontId,
    canvas: &mut Canvas<OpenGl>,
) {
    let posx = world.cache.posx(node.key());
    let posy = world.cache.posy(node.key());
    let width = world.cache.width(node.key());
    let height = world.cache.height(node.key());

    //println!("Draw node: {:?} {} {}", node.key(), posx, posy);

    //println!("Draw Node: {:?} at position: {} {}", node.key(), posx + parent_posx, posy+parent_posy);

    let red = world.store.red.get(&node.key()).unwrap_or(&0u8);
    let green = world.store.green.get(&node.key()).unwrap_or(&0u8);
    let blue = world.store.blue.get(&node.key()).unwrap_or(&0u8);

    let mut path = Path::new();
    path.rect(parent_posx + posx, parent_posy + posy, width, height);
    let paint = Paint::color(Color::rgb(*red, *green, *blue));
    canvas.fill_path(&mut path, paint);

    let mut paint = Paint::color(Color::black());
    paint.set_font_size(24.0);
    paint.set_text_align(Align::Center);
    paint.set_text_baseline(Baseline::Middle);
    paint.set_font(&vec![font]);
    let _ = canvas.fill_text(
        parent_posx + posx + width / 2.0,
        parent_posy + posy + height / 2.0,
        &node.key().0.to_string(),
        paint,
    );

    for child in node.children(&world.tree) {
        draw_node(child, world, posx + parent_posx, posy + parent_posy, font, canvas);
    }
}
