// Part of a very simple ECS for demonstration purposes only.

use crate::entity::Entity;

/// A type representing a tree of entities.
#[derive(Default, Debug)]
pub struct Tree {
    pub parent: Vec<Option<Entity>>,
    pub first_child: Vec<Option<Entity>>,
    pub next_sibling: Vec<Option<Entity>>,
    pub prev_sibling: Vec<Option<Entity>>,
}

impl Tree {
    /// Adds an entity to the tree with the given parent. A `None` parent means the node is a root node.
    pub fn add(&mut self, entity: Entity, parent: Option<Entity>) {
        if let Some(parent) = parent {
            if parent.index() >= self.parent.len() {
                return;
            }

            if entity.index() >= self.parent.len() {
                self.parent.resize(entity.index() + 1, None);
                self.first_child.resize(entity.index() + 1, None);
                self.next_sibling.resize(entity.index() + 1, None);
                self.prev_sibling.resize(entity.index() + 1, None);
            }

            self.parent[entity.index()] = Some(parent);
            self.first_child[entity.index()] = None;
            self.next_sibling[entity.index()] = None;
            self.prev_sibling[entity.index()] = None;

            if self.first_child[parent.index()].is_none() {
                self.first_child[parent.index()] = Some(entity);
            } else {
                let mut temp = self.first_child[parent.index()];

                loop {
                    if self.next_sibling[temp.unwrap().index()].is_none() {
                        break;
                    }

                    temp = self.next_sibling[temp.unwrap().index()];
                }

                self.next_sibling[temp.unwrap().index()] = Some(entity);
                self.prev_sibling[entity.index()] = temp;
            }
        } else {
            self.parent.push(None);
            self.first_child.push(None);
            self.next_sibling.push(None);
        }
    }

    pub fn remove(&mut self, entity: &Entity) {
        // Check if the entity to be removed exists in the tree.
        let entity_index = entity.0;
        if entity_index >= self.parent.len() {
            return;
        }

        // If the entity was is the first child of its parent then set its next sibling to be the new first child.
        if let Some(parent) = self.get_parent(entity).copied() {
            if self.get_prev_sibling(entity).is_none() {
                self.first_child[parent.index()] = self.get_next_sibling(entity).copied();
            }
        }

        // Set the next sibling of the previous sibling of the entity to the next sibling of the entity.
        // from:    [PS] -> [E] -> [NS]
        // to:      [PS] -> [NS]
        // where:   PS - Previous Sibling, E - Entity, NS - Next Sibling
        if let Some(prev_sibling) = self.get_prev_sibling(entity) {
            self.next_sibling[prev_sibling.index()] = self.get_next_sibling(entity).copied();
        }

        // Set the previous sibling of the next sibling of the entity to the previous sibling of the entity.
        // from:    [PS] <- [E] <- [NS]
        // to:      [PS] <- [NS]
        // where:   PS - Previous Sibling, E - Entity, NS - Next Sibling
        if let Some(next_sibling) = self.get_next_sibling(entity).copied() {
            self.prev_sibling[next_sibling.index()] = self.get_prev_sibling(entity);
        }

        // Set the next sibling, previous sibling and parent of the removed entity to None.
        self.next_sibling[entity_index] = None;
        self.prev_sibling[entity_index] = None;
        self.parent[entity_index] = None;
    }

    pub fn clear(&mut self) {
        self.parent.clear();
        self.first_child.clear();
        self.next_sibling.clear();
        self.prev_sibling.clear();
    }

    /// Returns the parent of the given entity if it exists.
    pub fn get_parent(&self, entity: &Entity) -> Option<&Entity> {
        self.parent.get(entity.index()).and_then(|parent| parent.as_ref())
    }

    /// Returns the first child of the given entity if it exists.
    pub fn get_first_child(&self, entity: &Entity) -> Option<&Entity> {
        self.first_child.get(entity.index()).and_then(|first_child| first_child.as_ref())
    }

    /// Returns the next sibling of the given entity if it exists.
    pub fn get_next_sibling(&self, entity: &Entity) -> Option<&Entity> {
        self.next_sibling.get(entity.index()).and_then(|prev_sibling| prev_sibling.as_ref())
    }

    /// Returns the previous sibling of the given entity if it exists.
    pub fn get_prev_sibling(&self, entity: &Entity) -> Option<Entity> {
        self.prev_sibling.get(entity.index()).and_then(|next_sibling| *next_sibling)
    }
}

pub struct DownwardIterator<'a> {
    tree: &'a Tree,
    current_node: Option<&'a Entity>,
}

impl<'a> DownwardIterator<'a> {
    /// Skip to next branch
    pub fn next_branch(&mut self) -> Option<&'a Entity> {
        let r = self.current_node;
        if let Some(current) = self.current_node {
            let mut temp = Some(current);
            while temp.is_some() {
                if let Some(sibling) = &self.tree.next_sibling[temp.unwrap().index()] {
                    self.current_node = Some(sibling);
                    return r;
                } else {
                    temp = self.tree.parent[temp.unwrap().index()].as_ref();
                }
            }
        } else {
            self.current_node = None;
        }

        None
    }
}

impl<'a> Iterator for DownwardIterator<'a> {
    type Item = &'a Entity;
    fn next(&mut self) -> Option<&'a Entity> {
        let r = self.current_node;

        if let Some(current) = self.current_node {
            if let Some(child) = &self.tree.first_child[current.index()] {
                self.current_node = Some(child);
            } else {
                let mut temp = Some(current);
                while temp.is_some() {
                    if let Some(sibling) = &self.tree.next_sibling[temp.unwrap().index()] {
                        self.current_node = Some(sibling);
                        return r;
                    } else {
                        temp = self.tree.parent[temp.unwrap().index()].as_ref();
                    }
                }

                self.current_node = None;
            }
        }

        r
    }
}

/// An iterator for iterating the children of an entity.
pub struct ChildIterator<'a> {
    pub tree: &'a Tree,
    pub current_node: Option<&'a Entity>,
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = &'a Entity;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity) = self.current_node {
            //self.current_node = self.tree.next_sibling[entity.index()].as_ref();
            self.current_node = self.tree.get_next_sibling(entity);
            return Some(entity);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;
    use crate::entity::{Entity, EntityManager};

    #[test]
    fn add_entity() {
        let mut tree = Tree::default();
        let mut entity_manager = EntityManager::default();

        let root = entity_manager.create();
        assert_eq!(root, Entity(0));

        tree.add(root, None);
    }
}
