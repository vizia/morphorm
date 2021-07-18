


use morphorm_ecs::{
    World,
};

use morphorm::*;

use winit::event::{
    Event,
    WindowEvent,
};
use winit::event_loop::{
    ControlFlow,
    EventLoop,
};
use winit::window::WindowBuilder;

use femtovg::{
    //CompositeOperation,
    renderer::OpenGl,
    Canvas,
    Color,
    Paint,
    Path,
};

fn main() {

    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(1000.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_left(root, Units::Pixels(0.0));
    world.set_top(root, Units::Pixels(0.0));

    let child = world.add(Some(root));
    world.set_width(child, Units::Percentage(50.0));
    world.set_height(child, Units::Stretch(1.0));

    // let _child2 = world.add(Some(root));
    // world.set_width(child, Units::Stretch(1.0));
    // world.set_height(child, Units::Stretch(1.0));
    // world.set_left(child, Units::Pixels(50.0));


    layout(&mut world.cache, &world.tree, &world.store);

    let computed_width = world.cache.width(&child);
    let computed_height = world.cache.height(&child);
    let computed_posx = world.cache.posx(&child);
    let computed_posy = world.cache.posy(&child);

    println!("Computed Width: {}", computed_width);
    println!("Computed Height: {}", computed_height);
    println!("Computed PosX: {}", computed_posx);
    println!("Computed PosY: {}", computed_posy);


    let el = EventLoop::new();

    let (renderer, windowed_context) = {
        use glutin::ContextBuilder;

        let wb = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(1000, 600))
            .with_title("Morphorm Demo");

        let windowed_context = ContextBuilder::new().with_vsync(false).build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let renderer =
            OpenGl::new(|s| windowed_context.get_proc_address(s) as *const _).expect("Cannot create renderer");

        (renderer, windowed_context)
    };

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

    el.run(move |event, _, control_flow| {
        #[cfg(not(target_arch = "wasm32"))]
        let window = windowed_context.window();

        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                    world.set_width(root, Units::Pixels(physical_size.width as f32));
                    world.set_height(root, Units::Pixels(physical_size.height as f32));

                    layout(&mut world.cache, &world.tree, &world.store);

                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {

                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgbf(0.3, 0.3, 0.32));

                for node in world.tree.down_iter() {
                    
                    let posx = world.cache.posx(&node);
                    let posy = world.cache.posy(&node);
                    let width = world.cache.width(&node);
                    let height = world.cache.height(&node);
                    
                    let red = world.store.red.get(&node).unwrap_or(&0u8);
                    let green = world.store.green.get(&node).unwrap_or(&0u8);
                    let blue = world.store.blue.get(&node).unwrap_or(&0u8);


                    let mut path = Path::new();
                    path.rect(posx, posy, width, height);
                    let paint = Paint::color(Color::rgb(*red,*green,*blue));
                    canvas.fill_path(&mut path, paint);
                }


                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw()
            }
            _ => (),
        }
    });
}
