use morphorm::Units;

use crate::entity::{Entity, EntityManager};
use crate::implementations::NodeCache;
use crate::store::Store;
use crate::tree::Tree;

use rand::Rng;


#[derive(Default)]
pub struct World {
    pub entity_manager: EntityManager,
    pub tree: Tree,
    pub store: Store,
    pub cache: NodeCache,
}

impl World {
    /// Add a node to the world with a specified parent node
    pub fn add(&mut self, parent: Option<Entity>) -> Entity {
        let entity = self.entity_manager.create();
        self.tree.add(entity, parent);
        self.cache.add(entity);

        let random_red: u8 = rand::thread_rng().gen();
        let random_green: u8 = rand::thread_rng().gen();
        let random_blue: u8 = rand::thread_rng().gen();

        self.store.red.insert(entity, random_red);
        self.store.green.insert(entity, random_green);
        self.store.blue.insert(entity, random_blue);
        entity
    }

    /// Set the desired width
    pub fn set_width(&mut self, entity: Entity, value: Units) {
        self.store.width.insert(entity, value);
    }

    /// Set the desired height
    pub fn set_height(&mut self, entity: Entity, value: Units) {
        self.store.height.insert(entity, value);
    }

    /// Set the desired left space
    pub fn set_left(&mut self, entity: Entity, value: Units) {
        self.store.left.insert(entity, value);
    }

    /// Set the desired right space
    pub fn set_right(&mut self, entity: Entity, value: Units) {
        self.store.right.insert(entity, value);
    }

    /// Set the desired top space
    pub fn set_top(&mut self, entity: Entity, value: Units) {
        self.store.top.insert(entity, value);
    }

    /// Set the desired bottom space
    pub fn set_bottom(&mut self, entity: Entity, value: Units) {
        self.store.bottom.insert(entity, value);
    }

    /// Set the desired left space
    pub fn set_child_left(&mut self, entity: Entity, value: Units) {
        self.store.child_left.insert(entity, value);
    }

    /// Set the desired right space
    pub fn set_child_right(&mut self, entity: Entity, value: Units) {
        self.store.child_right.insert(entity, value);
    }

    /// Set the desired top space
    pub fn set_child_top(&mut self, entity: Entity, value: Units) {
        self.store.child_top.insert(entity, value);
    }

    /// Set the desired bottom space
    pub fn set_child_bottom(&mut self, entity: Entity, value: Units) {
        self.store.child_bottom.insert(entity, value);
    }


    
}
