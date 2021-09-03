

use glutin::{error::OsError, event_loop::{ControlFlow, EventLoop}, window::{WindowBuilder, WindowId}};
use crate::{Entity, EventManager, State, Window, PropSet, Units};

type GEvent<'a, T> = glutin::event::Event<'a, T>;
type WinEvent<'a> = glutin::event::WindowEvent<'a>;


#[derive(Debug)]
pub enum AppError {
    WindowError(OsError),
}

#[derive(Debug)]
pub enum AppEvent {
    /// Emitted when a new window should be created by the application
    CreateWindow(Entity),
    /// Emiited when a new window should be destroyed by the application
    Destroy(Entity),
}
pub struct Application {
    state: State,
    event_loop: EventLoop<()>,
    window: (WindowId, Entity),
}

impl Application {
    pub fn new<F>(window_builder: WindowBuilder, app: F) -> Result<Application, AppError>
    where F: FnOnce(&mut State, Entity)
    {
        let mut state = State::default();

        let root = state.add(None);

        let event_loop = EventLoop::new();

        let mut window_widget = Window::new(window_builder);

        let window_id = window_widget.create(&mut state, root, &event_loop);

        state.components.insert(root, Box::new(window_widget));

        state.cache.layer.insert(root, 0);

        (app)(&mut state, root);


        Ok(Self {
            state,
            event_loop,
            window: (window_id, root),
        })
    }

    pub fn run(self) {
        let event_loop = self.event_loop;
        let mut state = self.state;

        let mut event_manager = EventManager::new(self.window.0);
        event_manager.windows.insert(self.window.0, self.window.1);

        event_loop.run(move |event, event_loop_window_target, control_flow|{
            *control_flow = ControlFlow::Wait;

            match event {
                GEvent::RedrawRequested(window_id) => {
                    event_manager.relayout(&mut state);
                    //let now = std::time::Instant::now();
                    event_manager.calculate_layers(&mut state, window_id);
                    event_manager.draw(&mut state, window_id);
                    //println!("{}", now.elapsed().as_millis());
                }

                GEvent::MainEventsCleared => {
                    event_manager.handle_events(&mut state, event_loop_window_target);
                    event_manager.rebuild_tree(&mut state);
                    
                }

                GEvent::WindowEvent{window_id, event} => {
                    match event {
                        WinEvent::CloseRequested => {
                            if let Some(entity) = event_manager.windows.get(&window_id) {
                                state.remove(*entity);
                                // Remove from tree and state
                                //state.components.remove(entity);
                            }

                            event_manager.windows.remove(&window_id);

                            

                            if event_manager.windows.is_empty() || event_manager.primary_window == window_id {
                                *control_flow = ControlFlow::Exit;
                            }
                        }

                        WinEvent::Resized(size) => {
                            if let Some(entity) = event_manager.windows.get(&window_id) {
                                entity.set_width(&mut state, Units::Pixels(size.width as f32)).set_height(&mut state, Units::Pixels(size.height as f32));
                            }
                        }

                        // WinEvent::MouseInput {
                        //     device_id,
                        //     state: button_state,
                        //     button,
                        //     modifiers,
                        // } => {
                            
                        // }

                        _=> {}
                    }
                }

                _=> {}
            }

        });
    }
}

// pub enum ContextCurrentWrapper {
//     PossiblyCurrent(ContextWrapper<PossiblyCurrent>),
//     NotCurrent(ContextWrapper<NotCurrent>),
// }

// type ContextId = usize;
// pub struct ContextTracker {
//     current: Option<ContextId>,
//     others: Vec<(ContextId, Option<ContextCurrentWrapper>)>,

// }