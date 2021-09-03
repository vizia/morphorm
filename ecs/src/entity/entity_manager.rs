use std::collections::VecDeque;

use crate::Entity;

const ENTITY_MAX: u32 = std::u32::MAX>>8;

const MINIMUM_FREE_INDICES: usize = 1024;

/// The entity manager is responsibe for creating, destroying, and reusing 
/// entities as well as checking if an entity is 'alive'.
pub(crate) struct EntityManager {
    generation: Vec<u8>,
    free_list: VecDeque<u32>,
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityManager {
    /// Creates a new entity manager
    pub(crate) fn new() -> Self {
        Self {
            generation: Vec::new(),
            free_list: VecDeque::with_capacity(MINIMUM_FREE_INDICES),
        }
    }

    /// Creates a new entity, reusing a destroyed entity if the number of reusable entities is greater than MINIMUM_FREE_INDICES.
    pub(crate) fn create(&mut self) -> Option<Entity> {
        let index = if self.free_list.len() > MINIMUM_FREE_INDICES {
            self.free_list.pop_front()
        } else {
            self.generation.push(0);
            let idx = (self.generation.len() - 1) as u32;
            assert!((idx as u32) < ENTITY_MAX, "Entity index exceeds maximum allowed value");
            Some(idx)
        };

        // Convert Option<u32> (index) to Option<Entity>
        index.map(|idx| Entity::new(idx, self.generation[idx as usize] as u32))
    }

    /// Returns true is the entity is alive
    pub(crate) fn is_alive(&self, entity: Entity) -> bool {
        self.generation[entity.index()] == entity.generation().unwrap()
    }

    /// Destroys an entity, adding it to the list of reusable entities
    pub(crate) fn destroy(&mut self, entity: Entity) {
        let index = entity.index() as u32;
        assert!(self.generation[index as usize] <= std::u8::MAX, "Entity generation exceeds maximum allowed value");
        self.generation[index as usize] += 1;
        self.free_list.push_back(index);
    }
}