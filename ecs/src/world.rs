// Part of a very simple ECS for demonstration purposes only.

use morphorm::{LayoutType, PositionType, Units};

use crate::entity::{Entity, EntityManager};
use crate::implementations::NodeCache;
use crate::store::Store;
use crate::tree::Tree;

use rand::Rng;

#[derive(Default, Debug, Clone, Copy)]
pub enum TextWrap {
    All,
    Hard,
    Soft,
    #[default]
    None,
}

/// An object which manages the state of an ECS application.
#[derive(Default)]
pub struct World {
    pub entity_manager: EntityManager,
    pub tree: Tree,
    pub store: Store,
    pub cache: NodeCache,
}

impl World {
    /// Add a node to the world with a specified parent node.
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

    /// Remove a node from the world.
    pub fn remove(&mut self, entity: Entity) {
        self.store.remove(entity);
        self.cache.remove(entity);
        self.tree.remove(&entity);
    }

    pub fn clear(&mut self) {
        self.entity_manager.reset();
        self.store.clear();
        self.cache.clear();
        self.tree.clear();
    }

    /// Set the desired layout type of the given entity.
    pub fn set_layout_type(&mut self, entity: Entity, value: LayoutType) {
        self.store.layout_type.insert(entity, value);
    }

    /// Set the desired position type of the given entity.
    pub fn set_position_type(&mut self, entity: Entity, value: PositionType) {
        self.store.position_type.insert(entity, value);
    }

    /// Set the desired width of the given entity.
    pub fn set_width(&mut self, entity: Entity, value: Units) {
        self.store.width.insert(entity, value);
    }

    /// Set the minimum width of the given entity.
    pub fn set_min_width(&mut self, entity: Entity, value: Units) {
        self.store.min_width.insert(entity, value);
    }

    /// Set the maximum width of the given entity.
    pub fn set_max_width(&mut self, entity: Entity, value: Units) {
        self.store.max_width.insert(entity, value);
    }

    /// Set the minimum height of the given entity.
    pub fn set_min_height(&mut self, entity: Entity, value: Units) {
        self.store.min_height.insert(entity, value);
    }

    /// Set the maximum height of the given entity.
    pub fn set_max_height(&mut self, entity: Entity, value: Units) {
        self.store.max_height.insert(entity, value);
    }

    /// Set the desired height of the given entity.
    pub fn set_height(&mut self, entity: Entity, value: Units) {
        self.store.height.insert(entity, value);
    }

    /// Set the desired left space of the given entity.
    pub fn set_left(&mut self, entity: Entity, value: Units) {
        self.store.left.insert(entity, value);
    }

    /// Set the desired right space of the given entity.
    pub fn set_right(&mut self, entity: Entity, value: Units) {
        self.store.right.insert(entity, value);
    }

    /// Set the desired top space of the given entity.
    pub fn set_top(&mut self, entity: Entity, value: Units) {
        self.store.top.insert(entity, value);
    }

    /// Set the desired bottom space of the given entity.
    pub fn set_bottom(&mut self, entity: Entity, value: Units) {
        self.store.bottom.insert(entity, value);
    }

    /// Set the desired child_space of the given entity.
    pub fn set_child_space(&mut self, entity: Entity, value: Units) {
        self.store.child_left.insert(entity, value);
        self.store.child_right.insert(entity, value);
        self.store.child_top.insert(entity, value);
        self.store.child_bottom.insert(entity, value);
    }

    /// Set the desired child_left space of the given entity.
    pub fn set_child_left(&mut self, entity: Entity, value: Units) {
        self.store.child_left.insert(entity, value);
    }

    /// Set the desired child_right space of the given entity.
    pub fn set_child_right(&mut self, entity: Entity, value: Units) {
        self.store.child_right.insert(entity, value);
    }

    /// Set the desired child_top space of the given entity.
    pub fn set_child_top(&mut self, entity: Entity, value: Units) {
        self.store.child_top.insert(entity, value);
    }

    /// Set the desired child_bottom space of the given entity.
    pub fn set_child_bottom(&mut self, entity: Entity, value: Units) {
        self.store.child_bottom.insert(entity, value);
    }

    /// Set the desired vertical (row) space between children of the given entity.
    pub fn set_row_between(&mut self, entity: Entity, value: Units) {
        self.store.row_between.insert(entity, value);
    }

    /// Set the desired horizontal (column) space between children of the given entity.
    pub fn set_col_between(&mut self, entity: Entity, value: Units) {
        self.store.col_between.insert(entity, value);
    }

    /// Set the minimum left space of the given entity.
    pub fn set_min_left(&mut self, entity: Entity, value: Units) {
        self.store.min_left.insert(entity, value);
    }

    /// Set the maximum left space of the given entity.
    pub fn set_max_left(&mut self, entity: Entity, value: Units) {
        self.store.max_left.insert(entity, value);
    }

    /// Set the minimum right space of the given entity.
    pub fn set_min_right(&mut self, entity: Entity, value: Units) {
        self.store.min_right.insert(entity, value);
    }

    /// Set the maximum right space of the given entity.
    pub fn set_max_right(&mut self, entity: Entity, value: Units) {
        self.store.max_right.insert(entity, value);
    }

    /// Set the minimum top space of the given entity.
    pub fn set_min_top(&mut self, entity: Entity, value: Units) {
        self.store.min_top.insert(entity, value);
    }

    /// Set the maximum top space of the given entity.
    pub fn set_max_top(&mut self, entity: Entity, value: Units) {
        self.store.max_top.insert(entity, value);
    }

    /// Set the minimum bottom space of the given entity.
    pub fn set_min_bottom(&mut self, entity: Entity, value: Units) {
        self.store.min_bottom.insert(entity, value);
    }

    /// Set the maximum bottom space of the given entity.
    pub fn set_max_bottom(&mut self, entity: Entity, value: Units) {
        self.store.max_bottom.insert(entity, value);
    }

    /// Set the content size function for the given entity.
    pub fn set_content_size(
        &mut self,
        entity: Entity,
        content: impl Fn(&Store, Option<f32>, Option<f32>) -> (f32, f32) + 'static,
    ) {
        self.store.content_size.insert(entity, Box::new(content));
    }

    pub fn set_visibility(&mut self, entity: Entity, visible: bool) {
        self.store.visible.insert(entity, visible);
    }

    /// Set the text to be displayed on the given entity.
    pub fn set_text(&mut self, entity: Entity, text: &str) {
        self.store.text.insert(entity, String::from(text));
    }

    /// Set whether the text should wrap for the given entity.
    pub fn set_text_wrap(&mut self, entity: Entity, text_wrap: TextWrap) {
        self.store.text_wrap.insert(entity, text_wrap);
    }

    /// Set all space and size properties of the given node to stretch.
    pub fn set_all_stretch(&mut self, entity: Entity) {
        self.set_left(entity, Units::Stretch(1.0));
        self.set_top(entity, Units::Stretch(1.0));
        self.set_right(entity, Units::Stretch(1.0));
        self.set_bottom(entity, Units::Stretch(1.0));
        self.set_width(entity, Units::Stretch(1.0));
        self.set_height(entity, Units::Stretch(1.0));
    }

    pub fn set_border(&mut self, entity: Entity, width: Units) {
        self.store.border_left.insert(entity, width);
        self.store.border_right.insert(entity, width);
        self.store.border_top.insert(entity, width);
        self.store.border_bottom.insert(entity, width);
    }
}
