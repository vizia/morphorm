use crate::Entity;

// Trait for accessing the entity id
// This allows for writing traits with methods which act on entities 
// without having to implement them directly on Entity
// This trait allows for accessing first first entity id of a tuple of entities 
pub trait AsEntity {
    fn entity(&self) -> Entity;
}

impl AsEntity for &Entity {
    fn entity(&self) -> Entity {
        **self
    }
}

impl AsEntity for &mut Entity {
    fn entity(&self) -> Entity {
        **self
    }
}

impl AsEntity for Entity {
    fn entity(&self) -> Entity {
        *self
    }
}

impl AsEntity for (Entity, Entity) {
    fn entity(&self) -> Entity {
        self.0
    }
}

impl AsEntity for (Entity, Entity, Entity) {
    fn entity(&self) -> Entity {
        self.0
    }
}

impl AsEntity for (Entity, Entity, Entity, Entity) {
    fn entity(&self) -> Entity {
        self.0
    }
}

impl AsEntity for (Entity, Entity, Entity, Entity, Entity) {
    fn entity(&self) -> Entity {
        self.0
    }
}