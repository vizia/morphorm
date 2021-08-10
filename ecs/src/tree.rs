
use crate::entity::Entity;

#[derive(Default, Debug)]
pub struct Tree {
    pub parent: Vec<Option<Entity>>,
    pub first_child: Vec<Option<Entity>>,
    pub next_sibling: Vec<Option<Entity>>,
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
            }

            self.parent[entity.index()] = Some(parent);
            self.first_child[entity.index()] = None;
            self.next_sibling[entity.index()] = None;

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
            }
        } else {
            self.parent.push(None);
            self.first_child.push(None);
            self.next_sibling.push(None);
        }
    }

    pub fn flatten(&self) -> Vec<Entity> {
        let iterator = DownwardIterator {
            tree: &self,
            current_node: Some(Entity(0)),
        };

        iterator.collect::<Vec<_>>()
    }

    pub fn get_first_child(&self, entity: Entity) -> Option<Entity> {

        if let Some(first_child) = self.first_child.get(entity.index()) {
            return *first_child;
        }

        None
    }

    pub fn get_next_sibling(&self, entity: Entity) -> Option<Entity> {

        if let Some(next_sibling) = self.next_sibling.get(entity.index()) {
            return *next_sibling;
        }

        None
    }
}

pub struct DownwardIterator<'a> {
    tree: &'a Tree,
    current_node: Option<Entity>,
}

impl<'a> Iterator for DownwardIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {

        let r = self.current_node;

        if let Some(current) = self.current_node {
            if let Some(child) = self.tree.first_child[current.index()] {
                self.current_node = Some(child);
            } else {
                let mut temp = Some(current);
                while temp.is_some() {
                    if let Some(sibling) = self.tree.next_sibling[temp.unwrap().index()] {
                        self.current_node = Some(sibling);
                        return r;
                    } else {
                        temp = self.tree.parent[temp.unwrap().index()];
                    }
                }

                self.current_node = None;
            }
        }

        return r;
    }
}

pub struct ChildIterator<'a> {
    pub tree: &'a Tree,
    pub current_node: Option<Entity>,
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity) = self.current_node {
            //self.current_node = self.tree.next_sibling[entity.index()].as_ref();
            self.current_node = self.tree.get_next_sibling(entity);
            return Some(entity);
        }

        None
    }
}
