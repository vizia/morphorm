use morphorm::Units;

use crate::entity::{Entity, EntityManager};
use crate::implementations::NodeCache;
use crate::store::Store;
use crate::tree::Tree;

use rand::Rng;


#[derive(Default)]
pub struct World {
    pub entity_manager: EntityManager,
    pub visual_tree: Tree,
    pub components: Store,
    pub node_cache: NodeCache,
}

impl World {
    pub fn add(&mut self, parent: Option<Entity>) -> Entity {
        let entity = self.entity_manager.create();
        self.visual_tree.add(entity, parent);
        self.node_cache.add(entity);

        let random_red: u8 = rand::thread_rng().gen();
        let random_green: u8 = rand::thread_rng().gen();
        let random_blue: u8 = rand::thread_rng().gen();

        self.components.red.insert(entity, random_red);
        self.components.green.insert(entity, random_green);
        self.components.blue.insert(entity, random_blue);
        entity
    }

    /// Set the desired width
    pub fn set_width(&mut self, entity: Entity, value: Units) {
        self.components.width.insert(entity, value);
    }

    /// Set the desired height
    pub fn set_height(&mut self, entity: Entity, value: Units) {
        self.components.height.insert(entity, value);
    }

    /// Set the desired left space
    pub fn set_left(&mut self, entity: Entity, value: Units) {
        self.components.left.insert(entity, value);
    }

    /// Set the desired right space
    pub fn set_right(&mut self, entity: Entity, value: Units) {
        self.components.right.insert(entity, value);
    }

    /// Set the desired top space
    pub fn set_top(&mut self, entity: Entity, value: Units) {
        self.components.top.insert(entity, value);
    }

    /// Set the desired bottom space
    pub fn set_bottom(&mut self, entity: Entity, value: Units) {
        self.components.bottom.insert(entity, value);
    }

    
}
