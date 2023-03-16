// Part of a very simple ECS for demonstration purposes only.

use std::fmt::Display;

use slotmap::{Key, KeyData};

/// An ID type used to set/get data from a store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(pub usize);

impl Default for Entity {
    fn default() -> Self {
        Entity(usize::MAX)
    }
}

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

    pub fn reset(&mut self) {
        self.count = 0;
    }
}

unsafe impl Key for Entity {
    fn data(&self) -> slotmap::KeyData {
        KeyData::from_ffi(self.0 as u64)
    }

    fn null() -> Self {
        Entity::default()
    }

    fn is_null(&self) -> bool {
        self.0 == usize::MAX
    }
}

impl From<KeyData> for Entity {
    fn from(value: KeyData) -> Self {
        Entity(value.as_ffi() as usize)
    }
}
