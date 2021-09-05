
use std::collections::{HashMap, VecDeque};

use femtovg::{FontId, ImageId};
// use morphorm::{Units, LayoutType, PositionType, Cache, GeometryChanged};

use crate::{NodeCache, ResourceManager, SparseSet, TreeExt, TreeOp};
use crate::entity::{Entity, EntityManager};
use crate::event::{Event, Component};
use crate::style::Style;
use crate::tree::Tree;

use rand::Rng;

#[derive(Default, Debug)]
pub struct Layer {
    pub posx: f32,
    pub posy: f32,
    pub width: f32,
    pub height: f32,
    pub image: Option<ImageId>,
}


#[derive(Default)]
pub struct State {
    // Creates and destroys entities
    pub(crate) entity_manager: EntityManager,
    // Describes the tree of entities
    pub tree: Tree,
    // List of tree operations used to sync the tree with event manager
    pub(crate) tree_ops: Vec<TreeOp>,
    // Style components for entities
    pub style: Style,
    // Cached data for entities
    pub cache: NodeCache,
    // Components
    pub components: HashMap<Entity, Box<dyn Component>>,
    // Queue of events
    pub event_queue: VecDeque<Event>,
    // List of entities which are windows
    pub windows: Vec<Entity>,

    // Temp
    pub font: Option<FontId>,


    pub resource_manager: ResourceManager,

    pub layers: SparseSet<Layer>,
    pub layer_free_list: VecDeque<Layer>,

}

impl State {
    /// Add an entity to the state with a specified parent entity
    pub fn add(&mut self, parent: Option<Entity>) -> Entity {
        
        let entity = self.entity_manager.create().expect("Failed to create entity!");
        
        self.tree.add(entity, parent).expect("Failed to add entity to tree");

        self.tree_ops.push(TreeOp::Add(entity, parent));

        self.cache.add(entity);

        let random_red: u8 = rand::thread_rng().gen();
        let random_green: u8 = rand::thread_rng().gen();
        let random_blue: u8 = rand::thread_rng().gen();

        self.style.red.insert(entity, random_red);
        self.style.green.insert(entity, random_green);
        self.style.blue.insert(entity, random_blue);            
    

        entity
    }

    pub fn remove(&mut self, entity: Entity) {

        if self.entity_manager.is_alive(entity) {
            let mut removed = Vec::new();
            for ent in entity.branch_iter(&self.tree) {
                self.tree_ops.push(TreeOp::Remove(ent));
                removed.push(ent);
            }

            for ent in removed.into_iter() {
                self.tree.remove(ent).expect("Failed to remove entity from tree");
                self.components.remove(&ent);
                self.entity_manager.destroy(entity);
            }            
        }
    }

    pub fn load_font(path: &str) {
        // Check if the font is already loaded
        // Add the font to the resource manager

    }
}
