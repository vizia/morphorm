use crate::Entity;


pub mod event_ext;
pub use event_ext::*;

pub mod event_manager;
pub use event_manager::*;

pub mod component;
pub use component::*;

use std::any::{Any, TypeId};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Propagation {
    Up,
    Direct,
}

pub trait Message: Any + std::fmt::Debug {
    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;

    // Perform the test
    //fn equals_a(&self, _: &dyn Message) -> bool;
}

impl dyn Message {
    // Check if a message is a certain type
    pub fn is<T: Message>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    // Casts a message to the specified type if the message is of that type
    pub fn downcast<T>(&mut self) -> Option<&mut T>
    where
        T: Message,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Message as *mut T)) }
        } else {
            None
        }
    }
}

// Implements message for any static type
impl<S: std::fmt::Debug + 'static> Message for S {
    fn as_any(&self) -> &dyn Any {
        self
    }

    // fn equals_a(&self, other: &dyn Message) -> bool {
    //     //other.as_any().type_id() == self.as_any().type_id()

    //     //println!("{:?} {:?}", other.as_any().type_id(), self.as_any().type_id());
    //     //println!("{:?} {:?}", other, self);

    //     other
    //         .as_any()
    //         .downcast_ref::<S>()
    //         .map_or(false, |a| self == a)
    // }
}

#[derive(Debug)]
pub struct Event {
    origin: Entity,
    target: Entity,
    propagation: Propagation,
    consumed: bool,
    message: Box<dyn Message>,    
}


