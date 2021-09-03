use crate::Entity;




pub struct SparseSet<T> {
    pub indices: Vec<usize>,
    pub data: Vec<T>,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, data: T) {
        if entity.index() >= self.indices.len() {
            self.indices.resize(entity.index() + 1, std::usize::MAX);
        }

        let data_index = self.indices[entity.index()];

        if data_index < self.data.len() {
            self.data[data_index] = data;
        } else {
            self.indices[entity.index()] = self.data.len();
            self.data.push(data);
        }
    }

    /// Return the data index of an entity if it exists
    pub fn data_index(&self, entity: Entity) -> Option<usize> {
        if entity.index() < self.indices.len() {
            let data_index = self.indices[entity.index()];
            if data_index < self.data.len() {
                return Some(data_index);
            }
        }

        None
    }

    pub fn set_data_index(&mut self, entity: Entity, data_index: usize) {
        // Make sure the data index is valid
        if data_index >= self.data.len() {
            return;
        }

        if entity.index() >= self.indices.len() {
            self.indices.resize(entity.index() + 1, std::usize::MAX);
        }

        self.indices[entity.index()] = data_index;
    } 

    pub fn get(&self, entity: Entity) -> Option<&T> {
        if entity.index() < self.indices.len() {
            let data_index = self.indices[entity.index()];
            if data_index < self.data.len() {
                return &self.data[data_index];
            }
        }

        None
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if entity.index() < self.indices.len() {
            let data_index = self.indices[entity.index()];
            if data_index < self.data.len() {
                return &mut self.data[data_index];
            }
        }

        None
    }
}

