use crate::{State, Entity, Widget, PropSet};
use std::marker::PhantomData;

use morphorm::{Units};

pub struct Builder<'a, W> {
    pub entity: Entity,
    pub state: &'a mut State,
    phantom: PhantomData<W>,
}

impl<'a, W: Widget> Builder<'a, W> {
    pub fn new(state: &'a mut State, entity: Entity) -> Self {
        Self {
            entity,
            state,
            phantom: PhantomData,
        }
    }

    pub(crate) fn build(self, widget: W) -> Entity {
        self.state.components.insert(self.entity, Box::new(widget));

        self.entity
    }

    pub fn set_unique_layer(self, value: bool) -> Self {
        self.state.style.unique_layer.insert(self.entity, value);

        self
    }

    pub fn set_left(self, value: Units) -> Self {
        self.entity.set_left(self.state, value);

        self
    }

    pub fn set_right(self, value: Units) -> Self {
        self.entity.set_right(self.state, value);

        self
    }

    pub fn set_top(self, value: Units) -> Self {
        self.entity.set_top(self.state, value);

        self
    }

    pub fn set_bottom(self, value: Units) -> Self {
        self.entity.set_bottom(self.state, value);

        self
    }

    pub fn set_space(self, value: Units) -> Self {
        self.entity.set_space(self.state, value);

        self
    }

    pub fn set_child_left(self, value: Units) -> Self {
        self.entity.set_child_left(self.state, value);

        self
    }

    pub fn set_child_right(self, value: Units) -> Self {
        self.entity.set_child_right(self.state, value);

        self
    }

    pub fn set_child_top(self, value: Units) -> Self {
        self.entity.set_child_top(self.state, value);

        self
    }

    pub fn set_child_bottom(self, value: Units) -> Self {
        self.entity.set_child_bottom(self.state, value);

        self
    }

    pub fn set_child_space(self, value: Units) -> Self {
        self.entity.set_child_space(self.state, value);

        self
    }

    pub fn set_width(self, value: Units) -> Self {
        self.entity.set_width(self.state, value);

        self
    }

    pub fn set_height(self, value: Units) -> Self {
        self.entity.set_height(self.state, value);

        self
    }
}