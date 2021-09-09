
use std::collections::HashMap;

use rand::Rng;


use femtovg::{Paint, Path, RenderTarget};
use glutin::{ContextWrapper, PossiblyCurrent, event_loop::EventLoopWindowTarget, window::WindowId};

use crate::{AppEvent, Canvas, CurrentContextWrapper, Entity, Propagation, State, Tree, TreeExt, TreeOp, Units, Window, state::Layer};

use morphorm::{Hierarchy, Cache};

// Step 0 - Cleanup unused resources
// Step 1 - Handle any events
// Step 2 - Create/Destroy windows
// Step 3 - Add/Remove elements from tree
// Step 4 - Apply styling
// Step 5 - Perform Relayout of changed entities
// Step 6 - Determine which layers widgets should be painted to
// Step 7 - Redraw entities which have changed into layers
// Step 8 - Composite layers into final image


pub struct EventManager {
    // Copy of the tree synced with the one in State
    pub tree: Tree,
    pub windows: HashMap<WindowId, Entity>,
    pub primary_window: WindowId,
}

impl EventManager {
    pub fn new(primary_window: WindowId) -> Self {
        Self {
            tree: Tree::new(),
            windows: HashMap::new(),
            primary_window,
        }
    }

    // Step 5 - Relayout
    pub fn relayout(&mut self, state: &mut State) {
        // Perform relayout of the  entire tree
        // TODO - can be done on a per window basis
        morphorm::layout(&mut state.cache, &state.tree, &state.style);
    }

    // Step 6 - Determine layers
    pub fn calculate_layers(&mut self, state: &mut State, window_id: WindowId) {

        if let Some(window) = self.windows.get(&window_id) {
            self.set_not_current(state, window_id);

            // Determine which layers widgets should be drawn into
            // as well as the position and size of those layers
            if let Some(mut window_component) = state.components.remove(window) {
                if let Some(window_widget) = window_component.downcast_mut::<Window>() {
                    if let Some(current_context_wrapper) = window_widget.handle.take() {
                        let new_windowed_context = match current_context_wrapper {

                            CurrentContextWrapper::NotCurrent(windowed_context) => {
                                let new_context = unsafe { windowed_context.make_current().unwrap()};

                                if let Some(layer) = state.layers.get_mut(*window) {
                                    layer.posx = layer.posx.min(state.cache.posx(*window) as usize);
                                    layer.posy = layer.posy.min(state.cache.posy(*window) as usize);
                                    layer.width = layer.width.max(state.cache.width(*window) as usize);
                                    layer.height = layer.height.max(state.cache.height(*window) as usize);

                                    if state.style.should_redraw.get(window) == Some(&true) {
                                        layer.needs_redraw = true;
                                        layer.needs_clear = true;
                                    }
                                }

                                self.calc_layers(state, *window, window_widget.canvas.as_mut().unwrap(), window_id);

                                // Reallocate layer images if the size has changed
                                for layer in state.layers.data.iter_mut() {
                                    if layer.window == window_id {
                                        //if layer.width != layer.image.width {
                                            //state.resource_manager.images.remove(layer.image);
                                        let flag = state.resource_manager.images.get(window_widget.canvas.as_mut().unwrap(), &mut layer.image, layer.width, layer.height);
                                        layer.needs_clear = flag;
                                        layer.needs_redraw = flag;
                                        //}
                                    }
                                }

                                window_widget.handle = Some(CurrentContextWrapper::PossiblyCurrent(new_context));
                            },

                            _=> {}
                        };
                    }
                }

                state.components.insert(*window, window_component);
            }
        
            
        
        }
   
    }

    // Step 8 - Composite layers into final image
    pub fn composite(&mut self, state: &mut State, window_id: WindowId) {

        // Redraw the layers
        if let Some(window) = self.windows.get(&window_id) {
            self.set_not_current(state, window_id);

            if let Some(mut window_component) = state.components.remove(window) {
                if let Some(window_widget) = window_component.downcast_mut::<Window>() {
                    if let Some(current_context_wrapper) = window_widget.handle.take() {
                        let new_windowed_context = match current_context_wrapper {

                            CurrentContextWrapper::NotCurrent(windowed_context) => {
                                let new_context = unsafe { windowed_context.make_current().unwrap()};
                                
                                let dpi_factor = new_context.window().scale_factor();
                                let size = new_context.window().inner_size();
                                
                                window_widget.canvas.as_mut().unwrap().set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                                window_widget.canvas.as_mut().unwrap().clear_rect(
                                    0,
                                    0,
                                    size.width as u32,
                                    size.height as u32,
                                    femtovg::Color::rgb(80, 80, 255),
                                );

                                self.draw_layers(state, window_widget.canvas.as_mut().unwrap(), window_id);

                                new_context.swap_buffers().expect("Failed to swap buffers.");

                                window_widget.handle = Some(CurrentContextWrapper::PossiblyCurrent(new_context));
                            },

                            // Unreachable because we made all the window contexts not current before this
                            _=> {}
                        };
                        
                    }
                }

                state.components.insert(*window, window_component);
            }
        }
   
    }


    // Step 7 - Redraw widgets into layers
    pub fn draw(&mut self, state: &mut State, window_id: WindowId) {
        if let Some(window) = self.windows.get(&window_id) {
            // Make all the other windows not current
            self.set_not_current(state, window_id);

            if let Some(mut window_component) = state.components.remove(window) {
                if let Some(window_widget) = window_component.downcast_mut::<Window>() {
                    if let Some(current_context_wrapper) = window_widget.handle.take() {
                        match current_context_wrapper {
                            CurrentContextWrapper::NotCurrent(windowed_context) => {
                                let new_context = unsafe { windowed_context.make_current().unwrap()};
                                
                                if let Some(layer) = state.layers.get_mut(*window) {
                                    // if layer.needs_clear {
                                    //     window_widget.canvas.as_mut().unwrap().clear_rect(
                                    //         0,
                                    //         0,
                                    //         layer.width as u32,
                                    //         layer.height as u32,
                                    //         femtovg::Color::rgb(255, 80, 80),
                                    //     );
                                    // }
                                }

                                self.draw_widgets(state, *window, window_widget, &new_context);
                                window_widget.handle = Some(CurrentContextWrapper::PossiblyCurrent(new_context));
                                
                            },

                            _=> {}
                        };
                    }
                }

                state.components.insert(*window, window_component);
            }
        }
    }

    // Step 2 - Sync the tree with the one in state
    pub fn rebuild_tree(&mut self, state: &mut State) {
        // Clone the list of tree operation from state
        let tree_operations = state.tree_ops.clone();
        state.tree_ops.clear();

        // Iterate through the tree operation and apply them to internal tree to sync it with the one in State
        for op in tree_operations.into_iter() {
            match op {
                TreeOp::Add(entity, parent) => {
                    self.tree.add(entity, parent).expect("Failed to add entity to tree.");
                }

                TreeOp::Remove(entity) => {
                    self.tree.remove(entity).expect("Failed to remove entity from tree.");
                }
            }
        }
    }

    // Step 1 - Handle any events
    pub fn handle_events(&mut self, state: &mut State, event_loop: &EventLoopWindowTarget<()>) {
        let mut event_queue = Vec::new(); 

        event_queue.extend(state.event_queue.drain(0..));

        // Loop over the events in the event queue
        'events: for event in event_queue.iter_mut() {
            //println!("Event: {:?}", event);

            // Skip events with no target unless they are set to propagate to all entities
            // if event.target == Entity::null() && event.propagation != Propagation::All {
            //     continue 'events;
            // }

            if let Some(app_event) = event.message.downcast() {
                match app_event {
                    AppEvent::CreateWindow(entity) => {

                        // Make all the other windows not current
                        for (_, other) in self.windows.iter() {
                            if *other != *entity {
                                if let Some(mut window_component) = state.components.remove(other) {
                                    if let Some(window_widget) = window_component.downcast_mut::<Window>() {
                                        if let Some(current_context_wrapper) = window_widget.handle.take() {
                                            let new_windowed_context = match current_context_wrapper {
                                                CurrentContextWrapper::PossiblyCurrent(windowed_context) => {
                                                    CurrentContextWrapper::NotCurrent(unsafe { windowed_context.make_not_current().unwrap()})
                                                }

                                                t => t,
                                            };
                                            window_widget.handle = Some(new_windowed_context);
                                        }
                                    }

                                    state.components.insert(*other, window_component);
                                }
                            }
                        } 


                        if let Some(mut window_component) = state.components.remove(&entity) {
                            if let Some(window_widget) = window_component.downcast_mut::<Window>() {
                                let window_id = window_widget.create(state, *entity, event_loop);
                                self.windows.insert(window_id, *entity);
                            }

                            state.components.insert(*entity, window_component);
                        }

                        continue 'events;
                    }

                    _=> {}
                }
            }

            // if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            //     match window_event {
            //         WindowEvent::Redraw => {
            //             needs_redraw = true;
            //         }

            //         _ => {}
            //     }
            // }

            // Define the target to prevent multiple mutable borrows error
            let target = event.target;

            // // A null entity as target means send event to all entities
            // if event.propagation == Propagation::All {
            //     for entity in self.tree.into_iter() {
            //         if let Some(mut component) = state.components.remove(&entity) {
            //             component.on_event_(state, entity, event);

            //             state.components.insert(entity, component);

            //             if event.consumed {
            //                 break;
            //             }
            //         }
            //     }
            //     continue 'events;
            // }

            // // Propagate down from root to target (not including target)
            // if event.propagation == Propagation::Down || event.propagation == Propagation::DownUp {
            //     // Construct the list of widgets to walk down by going up from the target
            //     let ancestors: Vec<Entity> = event
            //         .target
            //         .parent_iter(&self.tree)
            //         .collect::<Vec<Entity>>();

            //     // Walk down the list of ancestors
            //     for entity in ancestors.iter().rev() {
            //         // Skip the window
            //         if *entity == Entity::root() {
            //             continue;
            //         }

            //         // Stop before the target entity
            //         if *entity == event.target {
            //             break;
            //         }

            //         // Send event to all ancestors before the target
            //         if let Some(mut component) = state.components.remove(&entity) {
            //             component.on_event(state, *entity, event);

            //             state.components.insert(*entity, component);

            //             // Skip to the next event if the current event is consumed
            //             if event.consumed {
            //                 continue 'events;
            //             }
            //         }
            //     }
            // }

            // Direct
            //if event.propagation != Propagation::Fall {
                // Send event to target
                if let Some(mut component) = state.components.remove(&event.target) {
                    component.on_event(state, event.target, event);

                    state.components.insert(event.target, component);
                    // if let Some(test) = self.callbacks.get_mut(&event.target) {
                    //     (test)(component, state, event.target);
                    // }

                    if event.consumed {
                        continue 'events;
                    }
                }
            //}

            // Propagate up from target to root (not including target)
            if event.propagation == Propagation::Up {
                // Walk up the tree from parent to parent
                for entity in target.parent_iter(&self.tree) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    if let Some(mut component) = state.components.remove(&entity) {
                        component.on_event(state, entity, event);

                        state.components.insert(entity, component);
                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }

            // // Propagate down from target to leaf of current branch
            // if event.propagation == Propagation::Fall {
            //     // Walk tree from the target down the branch
            //     for entity in target.branch_iter(&self.tree) {
            //         // Skip the target entity
            //         if entity == event.target {
            //             continue;
            //         }

            //         // Send event to all entities after the target on the same branch
            //         if let Some(mut component) = state.components.remove(&entity) {
            //             component.on_event(state, entity, event);

            //             state.components.insert(entity, component);
            //             // Skip to the next event if the current event is consumed
            //             if event.consumed {
            //                 continue 'events;
            //             }
            //         }
            //     }
            // }
        }
    }


    fn set_not_current(&self, state: &mut State, window_id: WindowId) {
        for (_, other) in self.windows.iter() {
            if let Some(mut window_component) = state.components.remove(other) {
                if let Some(window_widget) = window_component.downcast_mut::<Window>() {
                    if let Some(current_context_wrapper) = window_widget.handle.take() {
                        let new_windowed_context = match current_context_wrapper {
                            CurrentContextWrapper::PossiblyCurrent(windowed_context) => {
                                CurrentContextWrapper::NotCurrent(unsafe { windowed_context.make_not_current().unwrap()})
                            }

                            t => t,
                        };
                        window_widget.handle = Some(new_windowed_context);
                    }
                }

                state.components.insert(*other, window_component);
            }
        } 
    }

    fn draw_widgets(&self, state: &mut State, window: Entity, window_widget: &mut Window, new_context: &ContextWrapper<PossiblyCurrent, glutin::window::Window>) {

        if let Some(first_child) = window.first_child(&self.tree) {
            let mut tree_iterator = first_child.tree_iter(&self.tree);

            while let Some(entity) = tree_iterator.next() {
                
                if entity.prev_sibling(&self.tree) == Some(window) {
                    break;
                }

                if let Some(mut component) = state.components.remove(&entity) {
                    if component.is_window() {
                        tree_iterator.next_branch(Some(entity));
                    } else {
                        if let Some(layer) = state.layers.get(entity) {
                            if layer.needs_redraw {
                                component.on_draw(state, entity, window_widget.canvas.as_mut().unwrap());
                            }
                        }
                    }

                    state.components.insert(entity, component);
                }
            }   
        }
    }

    fn draw_layers(&self, state: &mut State, canvas: &mut Canvas, window_id: WindowId) {

        
        canvas.set_render_target(RenderTarget::Screen);
        for layer in state.layers.data.iter_mut() {
            if layer.window == window_id {
                if let Some(image_id) = layer.image {
                    //println!("Composite layer: {} {} {} {} {:?}", layer.posx, layer.posy, layer.width, layer.height, layer.image);
                    let mut path = Path::new();
                    path.rect(layer.posx as f32, layer.posy as f32, layer.width as f32, layer.height as f32);
                    let mut paint = Paint::image(image_id, layer.posx as f32, layer.posy as f32, layer.width as f32, layer.height as f32, 0.0, 1.0);
                    canvas.fill_path(&mut path, paint);
                }                
            }

            layer.needs_redraw = false;
            layer.needs_clear = false;
            layer.posx = std::usize::MAX;
            layer.posy = std::usize::MAX;
            layer.width = 0;
            layer.height = 0;
        }
        canvas.flush();

        
    }

    fn calc_layers(&self, state: &mut State, window: Entity, canvas: &mut Canvas, window_id: WindowId) {
        
        if let Some(first_child) = window.first_child(&self.tree) {
            let mut tree_iterator = first_child.tree_iter(&self.tree);

            while let Some(entity) = tree_iterator.next() {
                if entity.prev_sibling(&self.tree) == Some(window) {
                    break;
                }

                if let Some(mut component) = state.components.remove(&entity) {
                    if component.is_window() {
                        tree_iterator.next_branch(Some(entity));
                    } else {
                        let desired_width = state.style.width.get(&entity).cloned().unwrap_or_default();
                        let desired_height = state.style.height.get(&entity).cloned().unwrap_or_default();

                        
            
                        if let Some(parent) = state.tree.parent(entity) {
                            // Safe to unwrap because this algorithm iterates down the tree and assigns a layer to each entity
                            let parent_layer_index = state.layers.get_index(parent).unwrap();

                            let width = state.cache.width(entity);
                            let height = state.cache.height(entity);

                            if let Some(unique_layer_flag) =  state.style.unique_layer.get(&entity) {
                                if *unique_layer_flag {
                                    
                                    if state.layers.get_index(entity) == Some(parent_layer_index) || state.layers.get(entity).is_none() {
                                        println!("Entity {} is on a unique layer", entity);
                                        //let mut image = None;
                                        //let flag= state.resource_manager.images.get(canvas, &mut image, width as usize, height as usize);
                                        state.layers.insert(entity, Layer {
                                            posx: std::usize::MAX,
                                            posy: std::usize::MAX,
                                            width: 0,
                                            height: 0,
                                            image: None,
                                            needs_redraw: true,
                                            needs_clear: true,
                                            window: window_id,
                                        });
                                    }
                                }
                            } else {
                                match desired_width {
                                    Units::Pixels(width) => {
                                        match desired_height {
                                            Units::Pixels(height) => {
                                                //println!("Entity {} should be on separate layer.", entity);
                                                // Size specified in pixels so widget can be drawn on a separate layer to its parent
                                                // First, check if there's already an assigned layer and whether it's equal to the parent layer
                                                // If it is equal then we can grab a layer from the resource manager and insert it into the set
                                                if state.layers.get_index(entity) == Some(parent_layer_index) || state.layers.get(entity).is_none() {
                                                    //println!("Created new layer for entity: {}", entity);
                                                    //let mut image = None;
                                                    //let flag= state.resource_manager.images.get(canvas, &mut image, width as usize, height as usize);
                                                    state.layers.insert(entity, Layer {
                                                        posx: std::usize::MAX,
                                                        posy: std::usize::MAX,
                                                        width: 0,
                                                        height: 0,
                                                        image: None,
                                                        needs_redraw: true,
                                                        needs_clear: true,
                                                        window: window_id,
                                                    });
                                                }
                
                                            }
                
                                            _=> {
                                                state.layers.set_data_index(entity, parent);
                                            }
                                        }
                                    }
                
                                    _=> {
                                        state.layers.set_data_index(entity, parent);
                                    }
                                }                                
                            }
            
            




                            if let Some(layer) = state.layers.get_mut(entity) {

                                //if layer.needs_reposition {
                                layer.posx = layer.posx.min(state.cache.posx(entity) as usize);
                                layer.posy = layer.posy.min(state.cache.posy(entity) as usize);
                                layer.width = layer.width.max(state.cache.width(entity) as usize);
                                layer.height = layer.height.max(state.cache.height(entity) as usize);

                                //println!("Entity: {} Layer {:?} px {} py {}", entity, layer.image, state.cache.posx(entity), state.cache.posy(entity) );
                                //}

                                if state.style.should_redraw.get(&entity) == Some(&true) {
                                    layer.needs_redraw = true;
                                    layer.needs_clear = true;
                                    //println!("Layer: {:?} needs clear", layer.image);
                                }
                            }

                            //println!("So entity: {} should be drawn to window: {:?}, on layer with image id: {:?}", entity, window_id, state.layers.get(entity).unwrap().image);

            
                            //state.cache.layer.insert(entity, current_layer_index);
            
                        }
                    }

                    

                    state.components.insert(entity, component);
                }
            }   
        }
    }
}


