use morphorm::{Units, LayoutType, PositionType};

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

    /// Set the desired layout type
    pub fn set_layout_type(&mut self, entity: Entity, value: LayoutType) {
        self.store.layout_type.insert(entity, value);
    }

    /// Set the desired position type
    pub fn set_position_type(&mut self, entity: Entity, value: PositionType) {
        self.store.position_type.insert(entity, value);
    }

    /// Set the desired width
    pub fn set_main(&mut self, entity: Entity, value: Units) {
        self.store.main.insert(entity, value);
    }

    /// Set the desired height
    pub fn set_cross(&mut self, entity: Entity, value: Units) {
        self.store.cross.insert(entity, value);
    }

    /// Set the desired left space
    pub fn set_main_before(&mut self, entity: Entity, value: Units) {
        self.store.main_before.insert(entity, value);
    }

    /// Set the desired right space
    pub fn set_main_after(&mut self, entity: Entity, value: Units) {
        self.store.main_after.insert(entity, value);
    }

    /// Set the desired top space
    pub fn set_cross_before(&mut self, entity: Entity, value: Units) {
        self.store.cross_before.insert(entity, value);
    }

    /// Set the desired bottom space
    pub fn set_cross_after(&mut self, entity: Entity, value: Units) {
        self.store.cross_after.insert(entity, value);
    }

    /// Set the desired child_ space
    pub fn set_child_space(&mut self, entity: Entity, value: Units) {
        self.store.child_main_before.insert(entity, value);
        self.store.child_main_after.insert(entity, value);
        self.store.child_cross_before.insert(entity, value);
        self.store.child_cross_after.insert(entity, value);
    }


    /// Set the desired child_left space
    pub fn set_child_main_before(&mut self, entity: Entity, value: Units) {
        self.store.child_main_before.insert(entity, value);
    }

    /// Set the desired child_right space
    pub fn set_child_main_after(&mut self, entity: Entity, value: Units) {
        self.store.child_main_after.insert(entity, value);
    }

    /// Set the desired child_top space
    pub fn set_child_cross_before(&mut self, entity: Entity, value: Units) {
        self.store.child_cross_before.insert(entity, value);
    }

    /// Set the desired child_bottom space
    pub fn set_child_cross_after(&mut self, entity: Entity, value: Units) {
        self.store.child_cross_after.insert(entity, value);
    }

    /// Set the desired space between rows
    pub fn set_main_between(&mut self, entity: Entity, value: Units) {
        self.store.main_between.insert(entity, value);
    }

    /// Set the desired space between columns
    pub fn set_cross_between(&mut self, entity: Entity, value: Units) {
        self.store.cross_between.insert(entity, value);
    }

    /// Set the desired grid rows
    pub fn set_grid_rows(&mut self, entity: Entity, value: Vec<Units>) {
        self.store.grid_rows.insert(entity, value);
    }

    /// Set the desired grid columns
    pub fn set_grid_cols(&mut self, entity: Entity, value: Vec<Units>) {
        self.store.grid_cols.insert(entity, value);
    }

    /// Set the desired grid row index
    pub fn set_row(&mut self, entity: Entity, index: usize, span: usize) {
        self.store.row_index.insert(entity, index);
        self.store.row_span.insert(entity, span);
    }

    /// Set the desired grid row span
    pub fn set_col(&mut self, entity: Entity, index: usize, span: usize) {
        self.store.col_index.insert(entity, index);
        self.store.col_span.insert(entity, span);
    }

    pub fn set_min_main(&mut self, entity: Entity, value: Units) {
        self.store.min_main.insert(entity, value);
    }

    pub fn set_min_cross(&mut self, entity: Entity, value: Units) {
        self.store.min_cross.insert(entity, value);
    }

    pub fn set_text(&mut self, entity: Entity, value: String) {
        self.store.text.insert(entity, value);
    }

    pub fn set_intrinsic_size<F>(&mut self, entity: Entity, f: F)
    where F: 'static + Fn(&Store, f32) -> f32
    {
        self.store.intrinsic_size.insert(entity, Box::new(f));
    }
    
}
