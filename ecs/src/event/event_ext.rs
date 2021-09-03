use crate::{State, Entity, AsEntity, Event, Message, Propagation};

/// Trait which provides methods on entities for sending events
pub trait EventExt: AsEntity {
    /// Sned an event with a message up the tree
    fn emit<M: Message>(&self, state: &mut State, message: M) {
        state.event_queue.push_back(Event{
            origin: self.entity(),
            target: self.entity(),
            propagation: Propagation::Up,
            message: Box::new(message),
            consumed: false,
        });
    }

    /// Send an event with a message to a specific target
    fn emit_to<M: Message>(&self, state: &mut State, target: Entity, message: M) {
        state.event_queue.push_back(Event{
            origin: self.entity(),
            target,
            propagation: Propagation::Direct,
            message: Box::new(message),
            consumed: false,
        });
    }
}

impl EventExt for Entity {}