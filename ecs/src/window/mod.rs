use femtovg::renderer::OpenGl;
use glutin::{ContextBuilder, ContextWrapper, NotCurrent, PossiblyCurrent, event_loop::EventLoopWindowTarget, window::{WindowId}};

// pub mod window_description;
// pub use window_description::*;

pub use glutin::window::WindowBuilder;

pub mod window_event;
pub use window_event::WindowEvent;

use crate::{AppEvent, Entity, EventExt, PositionType, PropSet, State, Units, Widget, Rect};


pub type Canvas = femtovg::Canvas<femtovg::renderer::OpenGl>;

pub enum CurrentContextWrapper {
    PossiblyCurrent(ContextWrapper<PossiblyCurrent, glutin::window::Window>),
    NotCurrent(ContextWrapper<NotCurrent, glutin::window::Window>),
}

#[derive(Default)]
pub struct Window {
    window_builder: WindowBuilder,
    pub handle: Option<CurrentContextWrapper>,
    pub canvas: Option<Canvas>,
}

impl Window {
    pub fn new(window_builder: WindowBuilder) -> Self {
        Self {
            window_builder,
            handle: None,
            canvas: None,
        }
    }

    pub fn create(&mut self, state: &mut State, entity: Entity, event_loop: &EventLoopWindowTarget<()>) -> WindowId {

        // let window_builder = WindowBuilder::new().with_title(&self.window_description.title).with_inner_size(PhysicalSize::new(
        //     300,
        //     300,
        // ));

        let windowed_context = ContextBuilder::new().build_windowed(self.window_builder.clone(), event_loop).expect("Failed to make window");

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let renderer = OpenGl::new(|s| windowed_context.context().get_proc_address(s) as *const _)
        .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let font = canvas.add_font("examples/common/Roboto-Regular.ttf").expect("Failed to load font file");

        if state.font.is_none() {
            state.font = Some(font);
        }

        let dpi_factor = windowed_context.window().scale_factor();
        let size = windowed_context.window().inner_size();

        entity
            .set_position_type(state, PositionType::SelfDirected)
            .set_left(state, Units::Pixels(0.0))
            .set_top(state, Units::Pixels(0.0))
            .set_width(state, Units::Pixels(size.width as f32))
            .set_height(state, Units::Pixels(size.height as f32));
        
        state.cache.rect.insert(entity, Rect {
            posx: 0.0,
            posy: 0.0,
            width: size.width as f32,
            height: size.height as f32,
        });

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            femtovg::Color::rgb(255, 80, 80),
        );

        let window_id = windowed_context.window().id();

        self.handle = Some(CurrentContextWrapper::PossiblyCurrent(windowed_context));
        self.canvas = Some(canvas);

        window_id

    }
}

impl Widget for Window {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity.emit(state, AppEvent::CreateWindow(entity));

        if let Some(window_size) = self.window_builder.window.inner_size {
            entity
                .set_width(state, Units::Pixels(window_size.to_physical::<u32>(1.0).width as f32))
                .set_height(state, Units::Pixels(window_size.to_physical::<u32>(1.0).height as f32));
        }

        entity
    }
}