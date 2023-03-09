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
            self.parent.push(None);
            self.first_child.push(None);
            self.next_sibling.push(None);
        }
    }

    pub fn flatten(&self) -> Vec<Entity> {
        let iterator = DownwardIterator { tree: &self, current_node: Some(&Entity(0)) };

        iterator.map(|item| *item).collect::<Vec<_>>()
    }

    pub fn get_parent(&self, entity: &Entity) -> Option<&Entity> {
        self.parent.get(entity.index()).map_or(None, |parent| parent.as_ref())
    }

    pub fn get_first_child(&self, entity: &Entity) -> Option<&Entity> {
        self.first_child.get(entity.index()).map_or(None, |first_child| first_child.as_ref())
    }

    pub fn get_next_sibling(&self, entity: &Entity) -> Option<&Entity> {
        self.next_sibling.get(entity.index()).map_or(None, |prev_sibling| prev_sibling.as_ref())
    }

    pub fn get_prev_sibling(&self, entity: Entity) -> Option<Entity> {
        self.prev_sibling.get(entity.index()).map_or(None, |next_sibling| *next_sibling)
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

        return None;
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

        return r;
    }
}

// pub struct UpwardIterator<'a> {
//     tree: &'a Tree,
//     current_node: Option<Entity>,
// }

// impl<'a> Iterator for UpwardIterator<'a> {
//     type Item = Entity;

//     // TODO - Needs Testing
//     fn next(&mut self) -> Option<Entity> {

//         let r = self.current_node;

//         if let Some(current) = self.current_node {

//             if let Some(prev_sibling) = self.tree.prev_sibling[current.index()] {
//                 let mut temp = Some(prev_sibling);
//                 while temp.is_some() {
//                     if let Some(last_child) = self.tree.get_last_child(temp.unwrap()) {
//                         temp = Some(last_child);
//                     } else {
//                         self.current_node = Some(prev_sibling);
//                         return r;
//                     }
//                 }
//             } else {
//                 self.current_node = self.tree.get_parent(current);
//             }
//         }

//         return r;
//     }
// }

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
