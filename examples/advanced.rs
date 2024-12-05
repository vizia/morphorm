use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin::display::GetGlDisplay;
use glutin::prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor};
use glutin::surface::{GlSurface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use glutin_winit::DisplayBuilder;

use raw_window_handle::HasRawWindowHandle;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use rand::Rng;

use femtovg::{renderer::OpenGl, Align, Baseline, Canvas, Color, FontId, Paint, Path};

use std::collections::HashMap;
use std::num::NonZeroU32;

use morphorm::Units::*;
use morphorm::*;

#[derive(Default, Clone)]
pub struct Widget {
    child: Vec<Widget>,
    width: Units,
    height: Units,
    min_width: Units,
    max_width: Units,
    min_height: Units,
    max_height: Units,
    left: Units,
    right: Units,
    top: Units,
    bottom: Units,
    padding_left: Units,
    padding_right: Units,
    padding_top: Units,
    padding_bottom: Units,
    vertical_gap: Units,
    horizontal_gap: Units,
    min_vertical_gap: Units,
    min_horizontal_gap: Units,
    max_vertical_gap: Units,
    max_horizontal_gap: Units,
    vertical_scroll: f32,
    horizontal_scroll: f32,
    alignment: Alignment,
    layout_type: LayoutType,
    position_type: PositionType,
    color: femtovg::Color,
    id: u32,
}

impl Widget {
    pub fn new(id: u32, width: Units, height: Units) -> Self {
        let random_red: u8 = rand::thread_rng().gen();
        let random_green: u8 = rand::thread_rng().gen();
        let random_blue: u8 = rand::thread_rng().gen();

        Self {
            id,
            color: femtovg::Color::rgb(random_red, random_green, random_blue),
            width,
            height,
            ..Default::default()
        }
    }
}

impl Node for Widget {
    type Store = ();
    type Tree = ();
    type ChildIter<'t> = std::slice::Iter<'t, Widget>;
    type CacheKey = u32;
    type SubLayout<'a> = ();

    fn children<'t>(&'t self, _tree: &'t Self::Tree) -> Self::ChildIter<'t> {
        self.child.iter()
    }

    fn key(&self) -> Self::CacheKey {
        self.id
    }

    fn visible(&self, _store: &Self::Store) -> bool {
        true
    }

    fn width(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.width)
    }

    fn height(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.height)
    }

    fn min_width(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.min_width)
    }

    fn max_width(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.max_width)
    }

    fn min_height(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.min_height)
    }

    fn max_height(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.max_height)
    }

    fn layout_type(&self, _store: &Self::Store) -> Option<morphorm::LayoutType> {
        Some(self.layout_type)
    }

    fn position_type(&self, _store: &Self::Store) -> Option<morphorm::PositionType> {
        Some(self.position_type)
    }

    fn left(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.left)
    }

    fn right(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.right)
    }

    fn top(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.top)
    }

    fn bottom(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.bottom)
    }

    fn content_size<'a>(
        &self,
        _store: &Self::Store,
        _sublayout: &mut Self::SubLayout<'a>,
        _parent_width: Option<f32>,
        _parent_height: Option<f32>,
    ) -> Option<(f32, f32)> {
        None
    }

    fn padding_left(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.padding_left)
    }

    fn padding_right(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.padding_right)
    }

    fn padding_top(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.padding_top)
    }

    fn padding_bottom(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.padding_bottom)
    }

    fn vertical_gap(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.vertical_gap)
    }

    fn horizontal_gap(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.horizontal_gap)
    }

    fn border_left(&self, _store: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    fn border_right(&self, _store: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    fn border_top(&self, _store: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    fn border_bottom(&self, _store: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    fn alignment(&self, _store: &Self::Store) -> Option<Alignment> {
        Some(self.alignment)
    }

    fn vertical_scroll(&self, _store: &Self::Store) -> Option<f32> {
        Some(self.vertical_scroll)
    }

    fn horizontal_scroll(&self, _store: &Self::Store) -> Option<f32> {
        Some(self.horizontal_scroll)
    }

    fn min_vertical_gap(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.min_vertical_gap)
    }

    fn min_horizontal_gap(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.min_horizontal_gap)
    }

    fn max_vertical_gap(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.max_vertical_gap)
    }

    fn max_horizontal_gap(&self, _store: &Self::Store) -> Option<Units> {
        Some(self.max_horizontal_gap)
    }
}

#[derive(Default)]
pub struct LayoutCache {
    rect: HashMap<u32, (f32, f32, f32, f32)>,
}

impl Cache for LayoutCache {
    type Node = Widget;

    fn set_bounds(&mut self, node: &Self::Node, posx: f32, posy: f32, width: f32, height: f32) {
        if let Some(rect) = self.rect.get_mut(&node.key()) {
            rect.0 = width;
            rect.1 = height;
            rect.2 = posx;
            rect.3 = posy;
        } else {
            self.rect.insert(node.key(), (width, height, posx, posy));
        }
    }

    fn width(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(&node.key()) {
            return rect.0;
        }

        0.0
    }

    fn height(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(&node.key()) {
            return rect.1;
        }

        0.0
    }

    fn posx(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(&node.key()) {
            return rect.2;
        }

        0.0
    }

    fn posy(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(&node.key()) {
            return rect.3;
        }

        0.0
    }
}

fn main() {
    let mut cache = LayoutCache::default();
    let mut root = Widget::new(0, Pixels(600.0), Pixels(600.0));
    root.child.push(Widget::new(1, Pixels(400.0), Pixels(400.0)));
    root.layout(&mut cache, &(), &(), &mut ());
    render(cache, root);
}

pub fn render(mut cache: LayoutCache, mut root: Widget) {
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

                    root.width = Units::Pixels(size.width as f32);
                    root.height = Units::Pixels(size.height as f32);

                    root.layout(&mut cache, &(), &(), &mut ());
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                    if input.virtual_keycode == Some(VirtualKeyCode::H) && input.state == ElementState::Pressed {
                        print_node(&root, &cache, &(), true, false, String::new());
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgbf(0.3, 0.3, 0.32));

                draw_node(&root, &cache, &mut canvas, font);

                canvas.flush();
                surface.swap_buffers(&context).unwrap();
            }
            Event::MainEventsCleared => window.request_redraw(),
            _ => (),
        }
    });
}

fn draw_node(node: &Widget, cache: &LayoutCache, canvas: &mut Canvas<OpenGl>, font: FontId) {
    let posx = cache.posx(node);
    let posy = cache.posy(node);
    let width = cache.width(node);
    let height = cache.height(node);

    let color = node.color;

    let mut path = Path::new();
    path.rect(posx, posy, width, height);
    let paint = Paint::color(color);
    canvas.fill_path(&mut path, &paint);

    let mut paint = Paint::color(Color::black());
    paint.set_font_size(24.0);
    paint.set_text_align(Align::Center);
    paint.set_text_baseline(Baseline::Middle);
    paint.set_font(&vec![font]);
    let _ = canvas.fill_text(posx + width / 2.0, posy + height / 2.0, &node.key().to_string(), &paint);

    for child in (&node).children(&()) {
        draw_node(child, cache, canvas, font);
    }
}
