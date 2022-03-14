#![allow(unused_variables)]

use crate::types::*;

/// A Node describes a visual element that can be positioned and sized
pub trait Node<'w>: Clone + Copy + std::fmt::Debug {
    /// A type representing an external store in case the position and size data is not be owned by the node itself (e.g. ECS)
    type Data;

    /// Get the layout type of the node
    ///
    /// Layout type determines how the children of the node will be positioned and sized
    /// - A Row layout type means that the child nodes will be positioned horizontally one after another
    /// - A Column layout type means that the child nodes will be positioned vertically one after another
    /// - A Grid layout type means that the children will be positioned based on the grid_rows and grid columns
    ///   as well as the child's row_index, col_index, row_span, and col_span properties.  
    fn layout_type(&self, store: &Self::Data) -> Option<LayoutType> {
        Some(LayoutType::Column)
    }

    /// Get the  position type of the node
    ///
    /// The position type of the node determines whether the node will be positioned in-line with its siblings or independently
    fn position_type(&self, store: &Self::Data) -> Option<PositionType> {
        Some(PositionType::ParentDirected)
    }

    /// Get the desired width of the node in units
    fn width(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Stretch(1.0))
    }

    /// Get the desired height of the node in units
    fn height(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Stretch(1.0))
    }

    /// Get the desired min_width of the node in units
    fn min_width(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    /// Get the desired min_height of the node in units
    fn min_height(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    /// Get the desired max_width of the node in units
    fn max_width(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    /// Get the desired max_height of the node in units
    fn max_height(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    /// Get the measured width of the node's content (not children) in pixels
    fn content_width(&self, store: &'_ Self::Data) -> Option<f32> {
        Some(0.0)
    }

    /// Get the measured height of the node's content (not children) in pixels
    fn content_height(&self, store: &'_ Self::Data) -> Option<f32> {
        Some(0.0)
    }

    /// Get the desired space to the left of the node in units
    fn left(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired space to the right of the node in units
    fn right(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired space above the node in units
    fn top(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired space below the node in units
    fn bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    /// Get the desired min_left of the node in units
    fn min_left(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired min_left of the node in units
    fn max_left(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired min_left of the node in units
    fn min_right(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired min_left of the node in units
    fn max_right(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired min_left of the node in units
    fn min_top(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired min_left of the node in units
    fn max_top(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired min_left of the node in units
    fn min_bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired min_left of the node in units
    fn max_bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    /// Get the desired space to the left of all child nodes in units
    ///
    /// The `child_left` property of the parent describes the space applied to the left of all child nodes which have a `left` property of Auto
    /// The `left` property on a child node, when not set to Auto, will override the `child_left` property for that child.
    fn child_left(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired space to the right of all child nodes in units
    ///
    /// The `child_right` property of the parent describes the space applied to the left of all child nodes which have a `right` property of Auto
    /// The `right` property on a child node, when not set to Auto, will override the `child_right` property for that child.
    fn child_right(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired space above all child nodes in units
    ///
    /// The `child_top` property of the parent describes the space applied to the left of all child nodes which have a `top` property of Auto
    /// The `top` property on a child node, when not set to Auto, will override the `child_top` property for that child.
    fn child_top(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    /// Get the desired space below all child nodes in units
    ///
    /// The `child_bottom` property of the parent describes the space applied to the left of all child nodes which have a `bottom` property of Auto
    /// The `bottom` property on a child node, when not set to Auto, will override the `child_bottom` property for that child.
    fn child_bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    /// Get the desired space between children in units when stacked in a column
    fn row_between(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    /// Get the desired space between children in units when stacked in a row
    fn col_between(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    /// Get the desired grid rows as a vector of units
    fn grid_rows(&self, store: &'_ Self::Data) -> Option<Vec<Units>> {
        Some(vec![])
    }

    /// Get the desired grid columns as a vector of units
    fn grid_cols(&self, store: &'_ Self::Data) -> Option<Vec<Units>> {
        Some(vec![])
    }

    /// Get the desired row_index of the node in units
    fn row_index(&self, store: &'_ Self::Data) -> Option<usize> {
        Some(0)
    }
    /// Get the desired col_index of the node in units
    fn col_index(&self, store: &'_ Self::Data) -> Option<usize> {
        Some(0)
    }
    fn row_span(&self, store: &'_ Self::Data) -> Option<usize> {
        Some(1)
    }
    fn col_span(&self, store: &'_ Self::Data) -> Option<usize> {
        Some(1)
    }
    fn border_left(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    fn border_right(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    fn border_top(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }
    fn border_bottom(&self, store: &'_ Self::Data) -> Option<Units> {
        Some(Units::Auto)
    }

    // these are "generic getters"
    fn width_height(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.width(store),
            Axis::Y => self.height(store),
        }
    }
    fn min_width_height(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.min_width(store),
            Axis::Y => self.min_height(store),
        }
    }
    fn max_width_height(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.max_width(store),
            Axis::Y => self.max_height(store),
        }
    }
    fn content_width_height(&self, store: &'_ Self::Data, axis: Axis) -> Option<f32> {
        match axis {
            Axis::X => self.content_width(store),
            Axis::Y => self.content_height(store),
        }
    }
    fn left_top(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.left(store),
            Axis::Y => self.top(store),
        }
    }
    fn right_bottom(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.right(store),
            Axis::Y => self.bottom(store),
        }
    }
    fn min_left_top(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.min_left(store),
            Axis::Y => self.min_top(store),
        }
    }
    fn max_left_top(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.max_left(store),
            Axis::Y => self.max_top(store),
        }
    }
    fn min_right_bottom(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.min_right(store),
            Axis::Y => self.min_bottom(store),
        }
    }
    fn max_right_bottom(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.max_right(store),
            Axis::Y => self.max_bottom(store),
        }
    }
    fn child_left_top(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.child_left(store),
            Axis::Y => self.child_top(store),
        }
    }
    fn child_right_bottom(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.child_right(store),
            Axis::Y => self.child_bottom(store),
        }
    }
    fn row_col_between(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.row_between(store),
            Axis::Y => self.col_between(store),
        }
    }
    fn grid_rows_cols(&self, store: &'_ Self::Data, axis: Axis) -> Option<Vec<Units>> {
        match axis {
            Axis::X => self.grid_rows(store),
            Axis::Y => self.grid_cols(store),
        }
    }
    fn row_col_index(&self, store: &'_ Self::Data, axis: Axis) -> Option<usize> {
        match axis {
            Axis::X => self.row_index(store),
            Axis::Y => self.col_index(store),
        }
    }
    fn row_col_span(&self, store: &'_ Self::Data, axis: Axis) -> Option<usize> {
        match axis {
            Axis::X => self.row_span(store),
            Axis::Y => self.col_span(store),
        }
    }
    fn border_left_top(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.border_left(store),
            Axis::Y => self.border_top(store),
        }
    }
    fn border_right_bottom(&self, store: &'_ Self::Data, axis: Axis) -> Option<Units> {
        match axis {
            Axis::X => self.border_right(store),
            Axis::Y => self.border_bottom(store),
        }
    }
}
