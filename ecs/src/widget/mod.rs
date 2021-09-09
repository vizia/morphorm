#![allow(dead_code)]

use std::any::{Any, TypeId};

use crate::{AsEntity, Component, State, Entity, Event, Canvas, Window};

pub mod builder;
pub use builder::*;

pub mod element;
pub use element::*;

use morphorm::Cache;

use femtovg::{Align, Baseline, Color, Paint, Path, RenderTarget};


#[allow(dead_code)]
pub trait Widget: 'static + Sized {

    /// The type returned by the `on_build` method. Must implement `AsEntity` 
    type Ret: AsEntity;
    type Data: Any;

    /// Adds the widget into state and returns the associated type Ret - an entity id or a tuple of entity ids
    fn build<F>(mut self, state: &mut State, parent: impl AsEntity, mut builder: F) -> Self::Ret
    where
        F: FnMut(Builder<Self>) -> Builder<Self>,
        Self: std::marker::Sized + 'static,
    {
        // Create a new entity
        let entity = state.add(Some(parent.entity()));

        //state.insert_event(Event::new(WindowEvent::ChildAdded(entity)).direct(parent.entity()));

        

        // Call the on_build function of the widget
        let ret = self.on_build(state, entity);

        // Call the builder closure
        builder(Builder::new(state, entity)).build(self);

        // Return the entity or entities returned by the on_build method
        ret
    }

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret;

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {}
    
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {}

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        draw_widget(state, entity, canvas);
    }
}

impl<T: Widget> Component for T {
    fn on_update(&mut self, state: &mut State, entity: Entity, data: &dyn Any) {
        if let Some(data) = data.downcast_ref::<<T as Widget>::Data>() {
            <T as Widget>::on_update(self, state, entity, data);
        }
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        <T as Widget>::on_event(self, state, entity, event);
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        <T as Widget>::on_draw(self, state, entity, canvas);
    }

    fn is_window(&self) -> bool {
        TypeId::of::<T>() == TypeId::of::<Window>()
    }
}

fn draw_widget(state: &mut State, entity: Entity, canvas: &mut Canvas) {
    
    if let Some(layer) = state.layers.get_mut(entity) {
        let posx = state.cache.posx(entity) - layer.posx as f32;
        let posy = state.cache.posy(entity) - layer.posy as f32;
        let width = state.cache.width(entity);
        let height = state.cache.height(entity);

        
        // layer.width = width as usize;
        // layer.height = height as usize;

        // if layer.image.is_none() {
        //     state.resource_manager.images.create(canvas, layer).expect("Failed to create layer image");
        // }

        if let Some(image_id) = layer.image {
            println!("Draw {:?} with {} {} to layer: {:?} with {} {}", entity, width, height, layer.image, layer.width, layer.height);

            // if let Ok((w, h)) = canvas.image_size(image_id) {
            //     println!("Image Layer Size: {} {}", w, h);
            // }

            canvas.set_render_target(RenderTarget::Image(image_id));
            if layer.needs_clear {
                //println!("Clear Layer");
                canvas.clear_rect(0, 0, layer.width as u32, layer.height as u32, Color::rgba(0, 0, 0, 0));
                layer.needs_clear = false;
            }

            let red = state.style.red.get(&entity).unwrap_or(&0u8);
            let green = state.style.green.get(&entity).unwrap_or(&0u8);
            let blue = state.style.blue.get(&entity).unwrap_or(&0u8);


            let mut path = Path::new();
            path.rect(posx, posy, width, height);
            let paint = Paint::color(Color::rgb(*red,*green,*blue));
            canvas.fill_path(&mut path, paint);

            //println!("Draw Path: {} {} {} {}", posx, posy, width, height);

            let mut paint = Paint::color(Color::black());
            paint.set_font_size(24.0);
            paint.set_text_align(Align::Center);
            paint.set_text_baseline(Baseline::Middle);
            paint.set_font(&vec![state.font.unwrap()]);
            canvas.fill_text(posx + width/2.0, posy + height/2.0, &entity.to_string(), paint).expect("Failed to render text.");
            
            //canvas.set_render_target(RenderTarget::Screen);
        } 

        //canvas.set_render_target(RenderTarget::Screen);
    }
}