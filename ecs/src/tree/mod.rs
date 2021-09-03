use crate::entity::Entity;

pub mod tree_op;
pub use tree_op::TreeOp;

pub mod tree_ext;
pub use tree_ext::*;

pub mod tree_iter;
pub use tree_iter::*;

const MIN_CAPACITY: usize = 1024;

#[derive(Debug, Clone, Copy)]
pub enum TreeError {
    /// The entity does not exist in the tree
    NoParent,
    /// Parent does not exist in the tree
    InvalidParent,
    /// Sibling does not exist in the tree
    InvalidSibling,
    /// Entity is null
    NullEntity,
    /// Desired sibling is already the sibling
    AlreadySibling,
    /// Desired first child id already the first child
    AlreadyFirstChild,
}

#[derive(Default, Debug, Clone)]
pub struct Tree {
    pub parent: Vec<Option<Entity>>,
    pub first_child: Vec<Option<Entity>>,
    pub next_sibling: Vec<Option<Entity>>,
    pub prev_sibling: Vec<Option<Entity>>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            parent: Vec::with_capacity(MIN_CAPACITY),
            first_child: Vec::with_capacity(MIN_CAPACITY),
            next_sibling: Vec::with_capacity(MIN_CAPACITY),
            prev_sibling: Vec::with_capacity(MIN_CAPACITY),
        }
    }
}

impl Tree {
    /// Adds an entity to the tree
    pub fn add(&mut self, entity: Entity, parent: Option<Entity>) -> Result<(), TreeError> {
        if let Some(parent) = parent {
            if parent.index() >= self.parent.len() {
                return Err(TreeError::NoParent);
            }

            // Resize tree storage to accomodate new entity if required
            // Vectors are resized up to the next multiple of MIN_CAPACITY > entity index to avoid
            // multiple reallocations when adding many entities which are outside of the vec range
            if entity.index() >= self.parent.len() {
                self.parent.resize(entity.index() + 1, None);
                self.first_child.resize(entity.index() + 1, None);
                self.next_sibling.resize(entity.index() + 1, None);
                self.prev_sibling.resize(entity.index() + 1, None); 
            }

            // Assign the parent and set the rest to None for now
            self.parent[entity.index()] = Some(parent);
            self.first_child[entity.index()] = None;
            self.next_sibling[entity.index()] = None;
            self.prev_sibling[entity.index()] = None;

            // Determine if the 
            if self.first_child[parent.index()] == None {
                self.first_child[parent.index()] = Some(entity);
            } else {
                let mut temp = self.first_child[parent.index()];

                loop {
                    if self.next_sibling[temp.unwrap().index()] == None {
                        break;
                    }

                    temp = self.next_sibling[temp.unwrap().index()];
                }

                self.next_sibling[temp.unwrap().index()] = Some(entity);
                self.prev_sibling[entity.index()] = temp;
            }
        } else {
            // If parent in None then this is a root node
            self.parent.push(None);
            self.first_child.push(None);
            self.next_sibling.push(None);
            self.prev_sibling.push(None);
        }

        Ok(())
    }

    /// Removes an entity from the tree
    /// The parent inherits its children
    pub fn remove(&mut self, entity: Entity) -> Result<(), TreeError> {
        // Check if the entity is null
        if entity == Entity::null() {
            return Err(TreeError::NullEntity);
        }

        // Check if the entity to be removed exists in the tree
        let entity_index = entity.index();

        if entity_index >= self.parent.len() {
            return Err(TreeError::NoParent);
        }

        // If the entity was is the first child of its parent then set its next sibling to be the new first child
        if let Some(parent) = self.get_parent(entity) {
            let first_child = self.get_first_child(parent);
            if first_child == Some(entity) {
                self.first_child[parent.index()] = self.get_next_sibling(entity);
            }
        }

        // Set the next sibling of the previous sibling of the entity to the next sibling of the entity
        // from:    [PS] -> [E] -> [NS] 
        // to:      [PS] -> [NS]
        // where:   PS - Previous Sibling, E - Entity, NS - Next Sibling
        if let Some(prev_sibling) = self.get_prev_sibling(entity) {
            self.next_sibling[prev_sibling.index()] = self.get_next_sibling(entity);
        }

        // Set the previous sibling of the next sibling of the entity to the previous sibling of the entity
        // from:    [PS] <- [E] <- [NS] 
        // to:      [PS] <- [NS]
        // where:   PS - Previous Sibling, E - Entity, NS - Next Sibling
        if let Some(next_sibling) = self.get_next_sibling(entity) {
            self.prev_sibling[next_sibling.index()] = self.get_prev_sibling(entity);
        }

        // Set the next sibling, previous sibling and parent of the removed entity to None
        self.next_sibling[entity_index] = None;
        self.prev_sibling[entity_index] = None;
        self.parent[entity_index] = None;

        Ok(())
    }

    /// Flatten the tree into a single vector of entities
    pub fn flatten(&self) -> Vec<Entity> {
        let iterator = DownIter {
            tree: &self,
            current_node: Some(Entity::new(0, 0)),
        };

        iterator.collect::<Vec<_>>()
    }

    /// Get the parent of an entity
    pub fn get_parent(&self, entity: Entity) -> Option<Entity> {
        self.parent.get(entity.index()).map_or(None, |parent| *parent)
    }

    /// Get the first child of an entity
    pub fn get_first_child(&self, entity: Entity) -> Option<Entity> {
        self.first_child.get(entity.index()).map_or(None, |first_child| *first_child)
    }

    /// Get the last child of an entity
    pub fn get_last_child(&self, entity: Entity) -> Option<Entity> {
        todo!()
    }

    /// Get the next sibling on an entity 
    pub fn get_next_sibling(&self, entity: Entity) -> Option<Entity> {
        self.next_sibling.get(entity.index()).map_or(None, |prev_sibling| *prev_sibling)
    }

    /// Get the previous sibling of an entity
    pub fn get_prev_sibling(&self, entity: Entity) -> Option<Entity> {
        self.prev_sibling.get(entity.index()).map_or(None, |next_sibling| *next_sibling)
    }
}


#[cfg(test)]
mod tests {
    use crate::entity::{Entity, EntityManager};
    use super::Tree;

    #[test]
    fn add_entity() {
        let mut tree = Tree::default();
        let mut entity_manager = EntityManager::default();

        let root = entity_manager.create();
        assert_eq!(root, Entity(0));

        tree.add(root, None);

        
    }
}