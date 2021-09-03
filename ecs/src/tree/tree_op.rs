use crate::Entity;

// Operations that can be performed on a tree
#[derive(Debug, Clone, Copy)]
pub enum TreeOp {
    // Add an entity with parent
    Add(Entity, Option<Entity>),
    // Remove
    Remove(Entity),
}