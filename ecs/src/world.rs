use morphorm::{Units, LayoutType, PositionType};

use crate::entity::{Entity, EntityManager};
use crate::implementations::NodeCache;
use crate::store::Store;
use crate::tree::Tree;

use rand::Rng;

/// An object which manages the state of an ECS application.
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
    pub fn set_width(&mut self, entity: Entity, value: Units) {
        self.store.width.insert(entity, value);
    }

    pub fn set_min_width(&mut self, entity: Entity, value: Units) {
        self.store.min_width.insert(entity, value);
    }

    pub fn set_max_width(&mut self, entity: Entity, value: Units) {
        self.store.max_width.insert(entity, value);
    }

    pub fn set_min_height(&mut self, entity: Entity, value: Units) {
        self.store.min_height.insert(entity, value);
    }

    pub fn set_max_height(&mut self, entity: Entity, value: Units) {
        self.store.max_height.insert(entity, value);
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

    /// Set the desired child_ space
    pub fn set_child_space(&mut self, entity: Entity, value: Units) {
        self.store.child_left.insert(entity, value);
        self.store.child_right.insert(entity, value);
        self.store.child_top.insert(entity, value);
        self.store.child_bottom.insert(entity, value);
    }

    /// Set the desired child_left space
    pub fn set_child_left(&mut self, entity: Entity, value: Units) {
        self.store.child_left.insert(entity, value);
    }

    /// Set the desired child_right space
    pub fn set_child_right(&mut self, entity: Entity, value: Units) {
        self.store.child_right.insert(entity, value);
    }

    /// Set the desired child_top space
    pub fn set_child_top(&mut self, entity: Entity, value: Units) {
        self.store.child_top.insert(entity, value);
    }

    /// Set the desired child_bottom space
    pub fn set_child_bottom(&mut self, entity: Entity, value: Units) {
        self.store.child_bottom.insert(entity, value);
    }

    /// Set the desired space between rows
    pub fn set_row_between(&mut self, entity: Entity, value: Units) {
        self.store.row_between.insert(entity, value);
    }

    /// Set the desired space between columns
    pub fn set_col_between(&mut self, entity: Entity, value: Units) {
        self.store.col_between.insert(entity, value);
    }

    pub fn set_min_left(&mut self, entity: Entity, value: Units) {
        self.store.min_left.insert(entity, value);
    }

    pub fn set_max_left(&mut self, entity: Entity, value: Units) {
        self.store.max_left.insert(entity, value);
    }

    pub fn set_min_right(&mut self, entity: Entity, value: Units) {
        self.store.min_right.insert(entity, value);
    }

    pub fn set_max_right(&mut self, entity: Entity, value: Units) {
        self.store.max_right.insert(entity, value);
    }

    pub fn set_min_top(&mut self, entity: Entity, value: Units) {
        self.store.min_top.insert(entity, value);
    }

    pub fn set_max_top(&mut self, entity: Entity, value: Units) {
        self.store.max_top.insert(entity, value);
    }

    pub fn set_min_bottom(&mut self, entity: Entity, value: Units) {
        self.store.min_bottom.insert(entity, value);
    }

    pub fn set_max_bottom(&mut self, entity: Entity, value: Units) {
        self.store.max_bottom.insert(entity, value);
    }

    pub fn set_content_size(&mut self, entity: Entity, content: impl Fn(f32) -> f32 + 'static) {
        self.store.content_size.insert(entity, Box::new(content));
    }

    pub fn set_all_stretch(&mut self, entity: Entity) {
        self.set_left(entity, Units::Stretch(1.0));
        self.set_top(entity, Units::Stretch(1.0));
        self.set_right(entity, Units::Stretch(1.0));
        self.set_bottom(entity, Units::Stretch(1.0));
        self.set_width(entity, Units::Stretch(1.0));
        self.set_height(entity, Units::Stretch(1.0));
    }

}
