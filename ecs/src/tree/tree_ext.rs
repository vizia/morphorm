
use crate::{AsEntity, BranchIter, DownIter, Entity, ParentIter, Tree};

pub trait TreeExt<'a>: AsEntity {
    /// Returns an iterator over the parents (ancestors) of an entity
    fn parent_iter(&self, tree: &'a Tree) -> ParentIter<'a> {
        ParentIter {
            tree,
            current_node: Some(self.entity()),
        }
    }

    /// Returns an iterator over the tree starting from the entity
    fn tree_iter(&self, tree: &'a Tree) -> DownIter<'a> {
        DownIter {
            tree,
            current_node: Some(self.entity()),
        }
    }

    /// Returns an iterator over a branch of the tree starting from the entity
    fn branch_iter(&self, tree: &'a Tree) -> BranchIter<'a> {
        BranchIter {
            tree,
            start_node: self.entity(),
            current_node: Some(self.entity()),
        }
    }

    /// Returns the first child of an entity if it exists, or None otherwise
    fn first_child(&self, tree: &'a Tree) -> Option<Entity> {
        tree.get_first_child(self.entity())
    }

    /// Returns the previous sibling of an entity if it exists, or None otherwise
    fn prev_sibling(&self, tree: &'a Tree) -> Option<Entity> {
        tree.get_prev_sibling(self.entity())
    }
}

impl<'a, T: AsEntity> TreeExt<'a> for T {}