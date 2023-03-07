// Part of a very simple ECS for demonstration purposes only.

use std::fmt::Display;


/// An ID type used to set/get data from a store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub usize);

impl Entity {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Manager for creating entities.
#[derive(Default)]
pub struct EntityManager {
    count: usize,
}

impl EntityManager {
    pub fn create(&mut self) -> Entity {
        self.count += 1;
        Entity(self.count - 1)
    }
}
