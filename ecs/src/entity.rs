// Part of a very simple ECS for demonstration purposes only.


/// An ID type used to set/get data from a store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub usize);

impl Entity {
    pub fn index(&self) -> usize {
        self.0
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
