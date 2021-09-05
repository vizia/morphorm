
use std::collections::HashMap;

use glutin::{ContextWrapper, PossiblyCurrent, event_loop::EventLoopWindowTarget, window::WindowId};

use crate::{AppEvent, Canvas, CurrentContextWrapper, Entity, Propagation, State, Tree, TreeExt, TreeOp, Window, Units};

use morphorm::Hierarchy;

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

        println!("Calculate Layers");
        

        for entity in self.tree.down_iter() {
            let desired_width = state.style.width.get(&entity).cloned().unwrap_or_default();
            let desired_height = state.style.height.get(&entity).cloned().unwrap_or_default();

            if let Some(parent) = state.tree.parent(entity) {
                let parent_layer = state.cache.layer.get(&parent).cloned().unwrap_or_default();
                let mut current_layer_index = parent_layer;
                match desired_width {
                    Units::Pixels(_) => {
                        match desired_height {
                            Units::Pixels(_) => {
                                // Pop a new layer off the free list
                                // Assign this entity to that layer
                                current_layer_index += 1;
                                
                                
                            }

                            _=> {}
                        }
                    }

                    _=> {}
                }

                state.cache.layer.insert(entity, current_layer_index);

            }

            println!("Paint entity: {} to layer: {:?}", entity, state.cache.layer.get(&entity));
        }
    }

    // Step 8 - Composite layers into final image
    pub fn composite(&mut self, state: &mut State, window_id: WindowId) {
        // Determine which layers belong to which window
        // Redraw the layers
         
    }


    // Step 7 - Redraw widgets into layers
    pub fn draw(&mut self, state: &mut State, window_id: WindowId) {
        if let Some(window) = self.windows.get(&window_id) {
            // Make all the other windows not current
            self.set_not_current(state, window_id);

            if let Some(mut window_component) = state.components.remove(window) {
                if let Some(window_widget) = window_component.downcast_mut::<Window>() {
                    if let Some(current_context_wrapper) = window_widget.handle.take() {
                        let new_windowed_context = match current_context_wrapper {
                            CurrentContextWrapper::PossiblyCurrent(windowed_context) => {
                                let new_context = unsafe { windowed_context.make_current().unwrap()};
                                
                                self.draw_widgets(state, *window, window_widget, &new_context);

                                CurrentContextWrapper::PossiblyCurrent(new_context)
                            }

                            CurrentContextWrapper::NotCurrent(windowed_context) => {
                                let new_context = unsafe { windowed_context.make_current().unwrap()};
                                
                                self.draw_widgets(state, *window, window_widget, &new_context);

                                CurrentContextWrapper::PossiblyCurrent(new_context)
                            },
                        };
                        window_widget.handle = Some(new_windowed_context);
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
        let dpi_factor = new_context.window().scale_factor();
        let size = new_context.window().inner_size();
        
        window_widget.canvas.as_mut().unwrap().set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        window_widget.canvas.as_mut().unwrap().clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            femtovg::Color::rgb(255, 80, 80),
        );
        
        //println!("Tree: {:?}", self.tree);
        if let Some(first_child) = window.first_child(&self.tree) {
            let mut tree_iterator = first_child.tree_iter(&self.tree);

            while let Some(entity) = tree_iterator.next() {
                
                if entity.prev_sibling(&self.tree) == Some(window) {
                    break;
                }

                if let Some(mut component) = state.components.remove(&entity) {
                    if component.is_window() {
                        let next = tree_iterator.next_branch(Some(entity));
                    } else {
                        component.on_draw(state, entity, window_widget.canvas.as_mut().unwrap());
                    }

                    

                    state.components.insert(entity, component);
                }
            }   
        }
        
        window_widget.canvas.as_mut().unwrap().flush();

        new_context.swap_buffers().expect("Failed to swap buffers.");
    }
}


