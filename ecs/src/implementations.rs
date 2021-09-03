use std::collections::HashMap;
use std::iter::Rev;

use morphorm::*;

use crate::DownIter;
use crate::entity::Entity;
use crate::style::Style;
use crate::tree::{ChildIter, Tree};

impl<'a> Node<'a> for Entity {
    type Data = Style;

    fn layout_type(&self, style: &'_ Self::Data) -> Option<LayoutType> {
        style.layout_type.get(self).cloned()
    }

    /// Get the  position type of the node
    fn position_type(&self, style: &'_ Self::Data) -> Option<PositionType> {
        style.position_type.get(self).cloned()
    }

    fn width(&self, style: &'_ Self::Data) -> Option<Units> {
        style.width.get(self).cloned()
    }

    fn height(&self, style: &'_ Self::Data) -> Option<Units> {
        style.height.get(self).cloned()
    }

    fn left(&self, style: &'_ Self::Data) -> Option<Units> {
        style.left.get(self).cloned()
    }
    fn right(&self, style: &'_ Self::Data) -> Option<Units> {
        style.right.get(self).cloned()
    }
    fn top(&self, style: &'_ Self::Data) -> Option<Units> {
        style.top.get(self).cloned()
    }
    fn bottom(&self, style: &'_ Self::Data) -> Option<Units> {
        style.bottom.get(self).cloned()
    }

    fn child_left(&self, style: &'_ Self::Data) -> Option<Units> {
        style.child_left.get(self).cloned()
    }

    fn child_right(&self, style: &'_ Self::Data) -> Option<Units> {
        style.child_right.get(self).cloned()
    }

    fn child_top(&self, style: &'_ Self::Data) -> Option<Units> {
        style.child_top.get(self).cloned()
    }

    fn child_bottom(&self, style: &'_ Self::Data) -> Option<Units> {
        style.child_bottom.get(self).cloned()
    }

    fn min_left(&self, style: &'_ Self::Data) -> Option<Units> {
        style.min_left.get(self).cloned()
    }

    fn min_right(&self, style: &'_ Self::Data) -> Option<Units> {
        style.min_right.get(self).cloned()
    }

    fn min_top(&self, style: &'_ Self::Data) -> Option<Units> {
        style.min_top.get(self).cloned()
    }

    fn min_bottom(&self, style: &'_ Self::Data) -> Option<Units> {
        style.min_bottom.get(self).cloned()
    }

    fn max_left(&self, style: &'_ Self::Data) -> Option<Units> {
        style.max_left.get(self).cloned()
    }

    fn max_right(&self, style: &'_ Self::Data) -> Option<Units> {
        style.max_right.get(self).cloned()
    }

    fn max_top(&self, style: &'_ Self::Data) -> Option<Units> {
        style.max_top.get(self).cloned()
    }

    fn max_bottom(&self, style: &'_ Self::Data) -> Option<Units> {
        style.max_bottom.get(self).cloned()
    }

    fn min_width(&self, style: &'_ Self::Data) -> Option<Units> {
        style.min_width.get(self).cloned()
    }

    fn max_width(&self, style: &'_ Self::Data) -> Option<Units> {
        style.max_width.get(self).cloned()
    }

    fn min_height(&self, style: &'_ Self::Data) -> Option<Units> {
        style.min_height.get(self).cloned()
    }

    fn max_height(&self, style: &'_ Self::Data) -> Option<Units> {
        style.max_height.get(self).cloned()
    }

    fn row_between(&self, style: &'_ Self::Data) -> Option<Units> {
        style.row_between.get(self).cloned()
    }

    fn col_between(&self, style: &'_ Self::Data) -> Option<Units> {
        style.col_between.get(self).cloned()
    }

    fn grid_rows(&self, style: &'_ Self::Data) -> Option<Vec<Units>> {
        style.grid_rows.get(self).cloned()
    }

    fn grid_cols(&self, style: &'_ Self::Data) -> Option<Vec<Units>> {
        style.grid_cols.get(self).cloned()
    }

    fn row_index(&self, style: &'_ Self::Data) -> Option<usize> {
        style.row_index.get(self).cloned()
    }

    fn col_index(&self, style: &'_ Self::Data) -> Option<usize> {
        style.col_index.get(self).cloned()
    }

    fn row_span(&self, style: &'_ Self::Data) -> Option<usize> {
        style.row_span.get(self).cloned()
    }

    fn col_span(&self, style: &'_ Self::Data) -> Option<usize> {
        style.col_span.get(self).cloned()
    }

    fn border_left(&self, style: &'_ Self::Data) -> Option<Units> {
        style.border.get(self).cloned()
    }

    fn border_right(&self, style: &'_ Self::Data) -> Option<Units> {
        style.border.get(self).cloned()
    }

    fn border_top(&self, style: &'_ Self::Data) -> Option<Units> {
        style.border.get(self).cloned()
    }

    fn border_bottom(&self, style: &'_ Self::Data) -> Option<Units> {
        style.border.get(self).cloned()
    }
}

// impl<'a,'b> Node<'b> for &'a Entity 
// where 'a: 'b
// {
//     type Data = style;
// }

impl<'a> Hierarchy<'a> for Tree {
    type Item = Entity;
    type DownIter = DownIter<'a>;
    type UpIter = Rev<std::vec::IntoIter<Entity>>;
    type ChildIter = ChildIter<'a>;

    fn up_iter(&'a self) -> Self::UpIter {
        self.flatten().into_iter().rev()
    }

    fn down_iter(&'a self) -> Self::DownIter {
        DownIter {
            tree: &self,
            current_node: Some(Entity::new(0, 0)),
        }
    }

    fn child_iter(&'a self, node: Self::Item) -> Self::ChildIter {
        let first_child = self.get_first_child(node);
        ChildIter {
            tree: self,
            current_node: first_child,
        }
    }

    fn parent(&self, node: Self::Item) -> Option<Self::Item> {
        if node.index() < self.parent.len() {
            return self.parent[node.index()]
        }

        None
    }

    fn is_first_child(&self, node: Self::Item) -> bool {
        if let Some(parent) = self.parent(node) {
            if let Some(first_child) = self.get_first_child(parent) {
                if first_child == node {
                    return true;
                } else {
                    return false;
                }
            }
        }

        false
    }

    fn is_last_child(&self, node: Self::Item) -> bool {
        if let Some(parent) = self.parent(node) {
            if let Some(mut temp) = self.get_first_child(parent) {
                while let Some(next_sibling) = self.get_next_sibling(temp) {
                    temp = next_sibling;
                }

                if temp == node {
                    return true;
                }
            }
        }

        false
    }
}

