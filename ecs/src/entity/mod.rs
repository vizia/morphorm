
pub(crate) mod entity_manager;
pub(crate) use entity_manager::*;

pub(crate) mod as_entity;
pub(crate) use as_entity::AsEntity;

use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

const ENTITY_INDEX_BITS: u32 = 24;
const ENTITY_INDEX_MASK: u32  = (1<<ENTITY_INDEX_BITS)-1;

const ENTITY_GENERATION_BITS: u32 = 8;
const ENTITY_GENERATION_MASK: u32 = (1<<ENTITY_GENERATION_BITS)-1;




/// An entity is an id used to reference to get/set properties in State.
/// Rather than having widgets own their data, all state is styled in a single database and
/// is styled and loaded using entities.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity(u32);

impl Default for Entity {
    fn default() -> Self {
        Entity::null()
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.index())
    }
}

impl Entity {
    /// Creates a null entity
    ///
    /// A null entity can be used as a placeholder within a widget struct but cannot be used to get/set properties
    pub fn null() -> Entity {
        Entity(std::u32::MAX)
    }

    /// Creates a root entity
    ///
    /// The root entity represents the main window and is alwys valid. 
    /// The root entity can be used to set properties on the window, such as background color, 
    /// as well as sending events to the window such as Restyle and Redraw events.
    pub fn root() -> Entity {
        Entity(0)
    }

    /// Creates a new entity with a given index and generation
    pub(crate) fn new(index: u32, generation: u32) -> Entity {
        Entity(index | generation << ENTITY_INDEX_BITS)
    }

    /// Returns true if the entity is null
    pub fn is_null(&self) -> bool {
        self.0 == std::u32::MAX
    }

    /// Returns the index of the entity
    // pub fn index(&self) -> Option<usize> {
    //     if self.0 < std::u32::MAX {
    //         Some((self.0 & ENTITY_INDEX_MASK) as usize)
    //     } else {
    //         None
    //     }
    // }

    /// Returns the generation of the entity
    pub fn generation(&self) -> Option<u8> {
        if self.0 < std::u32::MAX {
            Some(((self.0 >> ENTITY_INDEX_BITS) & ENTITY_GENERATION_MASK) as u8)
        } else {
            None
        }
    }

    pub(crate) fn index(&self) -> usize {
        (self.0 & ENTITY_INDEX_MASK) as usize
    }
}

