// Part of a very simple ECS for demonstration purposes only.

use morphorm::*;
use slotmap::SecondaryMap;

use crate::entity::Entity;
use crate::store::Store;
use crate::tree::{ChildIterator, Tree};

impl Node for Entity {
    type Store = Store;
    type Tree = Tree;
    type ChildIter<'t> = ChildIterator<'t>;
    type CacheKey = Entity;
    type SubLayout<'a> = ();

    fn key(&self) -> Self::CacheKey {
        *self
    }

    fn children<'t>(&self, tree: &'t Tree) -> Self::ChildIter<'t> {
        let current_node = tree.get_first_child(self);
        ChildIterator { tree, current_node }
    }

    fn visible(&self, store: &Store) -> bool {
        store.visible.get(*self).copied().unwrap_or(true)
    }

    fn layout_type(&self, store: &Store) -> Option<LayoutType> {
        store.layout_type.get(*self).copied()
    }

    fn position_type(&self, store: &Store) -> Option<PositionType> {
        store.position_type.get(*self).copied()
    }

    fn direction(&self, store: &Store) -> Option<Direction> {
        store.direction.get(*self).copied()
    }

    fn wrap(&self, store: &Store) -> Option<LayoutWrap> {
        store.wrap.get(*self).copied()
    }

    fn alignment(&self, store: &Store) -> Option<Alignment> {
        store.alignment.get(*self).copied()
    }

    fn width(&self, store: &Store) -> Option<Units> {
        store.width.get(*self).copied()
    }

    fn height(&self, store: &Store) -> Option<Units> {
        store.height.get(*self).copied()
    }

    fn left(&self, store: &Store) -> Option<Units> {
        store.left.get(*self).copied()
    }

    fn right(&self, store: &Store) -> Option<Units> {
        store.right.get(*self).copied()
    }

    fn top(&self, store: &Store) -> Option<Units> {
        store.top.get(*self).copied()
    }

    fn bottom(&self, store: &Store) -> Option<Units> {
        store.bottom.get(*self).copied()
    }

    fn content_size<'a>(
        &self,
        store: &Store,
        _sublayout: &mut (),
        width: Option<f32>,
        height: Option<f32>,
    ) -> Option<(f32, f32)> {
        store.content_size.get(*self).map(|t| (t)(store, width, height))
    }

    fn padding_left(&self, store: &Store) -> Option<Units> {
        store.padding_left.get(*self).copied()
    }

    fn padding_right(&self, store: &Store) -> Option<Units> {
        store.padding_right.get(*self).copied()
    }

    fn padding_top(&self, store: &Store) -> Option<Units> {
        store.padding_top.get(*self).copied()
    }

    fn padding_bottom(&self, store: &Store) -> Option<Units> {
        store.padding_bottom.get(*self).copied()
    }

    fn vertical_gap(&self, store: &Store) -> Option<Units> {
        store.vertical_gap.get(*self).copied()
    }

    fn horizontal_gap(&self, store: &Store) -> Option<Units> {
        store.horizontal_gap.get(*self).copied()
    }

    fn vertical_scroll(&self, store: &Store) -> Option<f32> {
        store.vertical_scroll.get(*self).copied()
    }

    fn horizontal_scroll(&self, store: &Store) -> Option<f32> {
        store.horizontal_scroll.get(*self).copied()
    }

    fn min_width(&self, store: &Store) -> Option<Units> {
        store.min_width.get(*self).copied()
    }

    fn max_width(&self, store: &Store) -> Option<Units> {
        store.max_width.get(*self).copied()
    }

    fn min_height(&self, store: &Store) -> Option<Units> {
        store.min_height.get(*self).copied()
    }

    fn max_height(&self, store: &Store) -> Option<Units> {
        store.max_height.get(*self).copied()
    }

    fn border_left(&self, store: &Store) -> Option<Units> {
        store.border_left.get(*self).copied()
    }

    fn border_right(&self, store: &Store) -> Option<Units> {
        store.border_right.get(*self).copied()
    }

    fn border_top(&self, store: &Store) -> Option<Units> {
        store.border_top.get(*self).copied()
    }

    fn border_bottom(&self, store: &Store) -> Option<Units> {
        store.border_bottom.get(*self).copied()
    }

    fn min_horizontal_gap(&self, store: &Store) -> Option<Units> {
        store.min_horizontal_gap.get(*self).copied()
    }

    fn max_horizontal_gap(&self, store: &Store) -> Option<Units> {
        store.max_horizontal_gap.get(*self).copied()
    }

    fn min_vertical_gap(&self, store: &Store) -> Option<Units> {
        store.min_vertical_gap.get(*self).copied()
    }

    fn max_vertical_gap(&self, store: &Store) -> Option<Units> {
        store.max_vertical_gap.get(*self).copied()
    }

    fn grid_columns(&self, store: &Self::Store) -> Option<Vec<Units>> {
        store.grid_columns.get(*self).cloned()
    }

    fn grid_rows(&self, store: &Self::Store) -> Option<Vec<Units>> {
        store.grid_rows.get(*self).cloned()
    }

    fn column_start(&self, store: &Self::Store) -> Option<usize> {
        store.column_start.get(*self).copied()
    }

    fn row_start(&self, store: &Self::Store) -> Option<usize> {
        store.row_start.get(*self).copied()
    }

    fn column_span(&self, store: &Self::Store) -> Option<usize> {
        store.column_span.get(*self).copied()
    }

    fn row_span(&self, store: &Self::Store) -> Option<usize> {
        store.row_span.get(*self).copied()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub posx: f32,
    pub posy: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default, Debug)]
pub struct NodeCache {
    // Computed size and position of nodes.
    pub rect: SecondaryMap<Entity, Rect>,
    // Pass-scoped memoized layout sizes.
    layout_memo: SecondaryMap<Entity, LayoutMemo>,
    layout_pass: u64,
    cross_pass_memo_enabled: bool,
    layout_revision: u64,
}

#[derive(Default, Debug, Clone, Copy)]
struct LayoutMemo {
    parent_layout_type: LayoutType,
    parent_main: f32,
    parent_cross: f32,
    size: Size,
    pass: u64,
    revision: u64,
    valid: bool,
}

#[inline]
fn same_f32(a: f32, b: f32) -> bool {
    a.to_bits() == b.to_bits()
}

impl NodeCache {
    pub fn enable_cross_pass_memoization(&mut self, enabled: bool) {
        self.cross_pass_memo_enabled = enabled;
    }

    pub fn set_layout_revision(&mut self, revision: u64) {
        self.layout_revision = revision;
    }

    pub fn bump_layout_revision(&mut self) {
        self.layout_revision = self.layout_revision.wrapping_add(1);
    }

    pub fn add(&mut self, entity: Entity) {
        self.rect.insert(entity, Default::default());
        self.layout_memo.insert(entity, Default::default());
    }

    pub fn remove(&mut self, entity: Entity) {
        self.rect.remove(entity);
        self.layout_memo.remove(entity);
    }

    pub fn clear(&mut self) {
        self.rect.clear();
        self.layout_memo.clear();
        self.layout_pass = 0;
        self.layout_revision = 0;
    }

    pub fn bounds(&self, entity: Entity) -> Option<&Rect> {
        self.rect.get(entity)
    }
}

impl Cache for NodeCache {
    type Node = Entity;

    fn begin_layout_pass(&mut self) {
        self.layout_pass = self.layout_pass.wrapping_add(1);
    }

    fn get_layout_result(
        &self,
        node: &Self::Node,
        parent_layout_type: LayoutType,
        parent_main: f32,
        parent_cross: f32,
    ) -> Option<Size> {
        let memo = self.layout_memo.get(*node)?;
        if !memo.valid {
            return None;
        }

        let pass_match = memo.pass == self.layout_pass;
        let revision_match = self.cross_pass_memo_enabled && memo.revision == self.layout_revision;
        if !pass_match && !revision_match {
            return None;
        }

        if memo.parent_layout_type == parent_layout_type
            && same_f32(memo.parent_main, parent_main)
            && same_f32(memo.parent_cross, parent_cross)
        {
            Some(memo.size)
        } else {
            None
        }
    }

    fn set_layout_result(
        &mut self,
        node: &Self::Node,
        parent_layout_type: LayoutType,
        parent_main: f32,
        parent_cross: f32,
        size: Size,
    ) {
        if let Some(memo) = self.layout_memo.get_mut(*node) {
            memo.parent_layout_type = parent_layout_type;
            memo.parent_main = parent_main;
            memo.parent_cross = parent_cross;
            memo.size = size;
            memo.pass = self.layout_pass;
            memo.revision = self.layout_revision;
            memo.valid = true;
        }
    }

    fn set_bounds(&mut self, node: &Self::Node, posx: f32, posy: f32, width: f32, height: f32) {
        if let Some(rect) = self.rect.get_mut(*node) {
            rect.posx = posx;
            rect.posy = posy;
            rect.width = width;
            rect.height = height;
        }
    }

    fn width(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(*node) {
            return rect.width;
        }

        0.0
    }

    fn height(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(*node) {
            return rect.height;
        }

        0.0
    }

    fn posx(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(*node) {
            return rect.posx;
        }

        0.0
    }

    fn posy(&self, node: &Self::Node) -> f32 {
        if let Some(rect) = self.rect.get(*node) {
            return rect.posy;
        }

        0.0
    }
}
