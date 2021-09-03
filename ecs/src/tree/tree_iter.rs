
use crate::{Entity, Tree};
use morphorm::Hierarchy;

// Various iterators for the tree


/// Iterator which iterates from parent to parent
pub struct ParentIter<'a> {
    pub tree: &'a Tree,
    pub current_node: Option<Entity>,
}

impl<'a> Iterator for ParentIter<'a> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity) = self.current_node {
            self.tree.parent(entity)
        } else {
            None
        }
    }
}

/// Iterator which iterates down a branch
pub struct BranchIter<'a> {
    pub tree: &'a Tree,
    pub start_node: Entity,
    pub current_node: Option<Entity>,
}

impl<'a> Iterator for BranchIter<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        let r = self.current_node;

        if let Some(current) = self.current_node {
            if let Some(child) = self.tree.get_first_child(current) {
                self.current_node = Some(child);
            } else {
                if self.current_node != Some(self.start_node) {
                    let mut temp = Some(current);
                    while temp.is_some() {
                        if let Some(sibling) =
                            self.tree.get_next_sibling(temp.unwrap())
                        {
                            self.current_node = Some(sibling);
                            return r;
                        } else {
                            temp = self.tree.get_parent(temp.unwrap());
                            if Some(self.start_node) == temp {
                                self.current_node = None;
                                temp = None;
                                //break;
                            }
                        }
                    }
                }

                self.current_node = None;
            }
        }

        return r;
    }
}

pub struct DownIter<'a> {
    pub tree: &'a Tree,
    pub current_node: Option<Entity>,
}

impl<'a> DownIter<'a> {
    /// Skip to next branch
    pub fn next_branch(&mut self, entity: Option<Entity>) -> Option<Entity> {

        let r = entity;
        if let Some(current) = entity {
            let mut temp = Some(current);
            while temp.is_some() {
                if let Some(sibling) = self.tree.next_sibling[temp.unwrap().index()]
                {
                    self.current_node = Some(sibling);
                    return r;
                } else {
                    temp = self.tree.parent[temp.unwrap().index()];
                }
            }
        } else {
            self.current_node = None;
        }

        self.current_node = None;
        return None;
    }
}

impl<'a> Iterator for DownIter<'a> {
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

pub struct UpIter<'a> {
    tree: &'a Tree,
    current_node: Option<Entity>,
}

impl<'a> Iterator for UpIter<'a> {
    type Item = Entity;

    // TODO - Needs Testing
    fn next(&mut self) -> Option<Entity> {

        let r = self.current_node;

        if let Some(current) = self.current_node {

            if let Some(prev_sibling) = self.tree.prev_sibling[current.index()] {
                let mut temp = Some(prev_sibling);
                while temp.is_some() {
                    if let Some(last_child) = self.tree.get_last_child(temp.unwrap()) {
                        temp = Some(last_child);
                    } else {
                        self.current_node = Some(prev_sibling);
                        return r;
                    }
                }
            } else {
                self.current_node = self.tree.get_parent(current);
            }
        }

        return r;
    }
}

pub struct ChildIter<'a> {
    pub tree: &'a Tree,
    pub current_node: Option<Entity>,
}

impl<'a> Iterator for ChildIter<'a> {
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