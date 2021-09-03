
use crate::{State, Entity, Widget};

pub struct Element {

}

impl Element {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Widget for Element {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}