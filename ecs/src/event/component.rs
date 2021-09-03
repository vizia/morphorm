
use std::any::{Any, TypeId};

use crate::{State, Entity, Event};

use crate::window::Canvas;

pub trait Component: Any {
    
    fn on_update(&mut self, state: &mut State, entity: Entity, data: &dyn Any) {}
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {}
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {}
    fn is_window(&self) -> bool {false}
}

impl dyn Component {
    // Check if a message is a certain type
    pub fn is<T: Component + 'static>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Component + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Component as *mut T)) }
        } else {
            None
        }
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Component + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Component as *const T)) }
        } else {
            None
        }
    }
}