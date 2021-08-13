use std::alloc::Layout;

use crate::units::*;

/// A Node describes a visual element that can be positioned and sized
pub trait Node<'w>: Clone + Copy + std::fmt::Debug {
    /// A type representing an external store in case the position and size data is not be owned by the node itself (e.g. ECS)
    type Data;

    /// Return true if the node is visibile
    ///
    /// Visibility in this case refers to the visibility of the node to the layout system,
    /// not necessarily to the render system. For example, a node could be visually invisible
    /// but still has its position and size determined by the layout system.
    fn is_visible(&self, store: &'_ Self::Data) -> bool {
        true
    }

    /// Get the layout type of the node
    ///
    /// Layout type determines how the children of the node will be positioned and sized
    /// - A Row layout type means that the child nodes will be positioned horizontally one after another
    /// - A Column layout type means that the child nodes will be positioned vertically one after another
    /// - A Grid layout type means that the children will be positioned based on the grid_rows and grid columns
    ///   as well as the child's row_index, col_index, row_span, and col_span properties.  
    fn layout_type(&self, store: &'_ Self::Data) -> Option<LayoutType> {
        Some(LayoutType::Column)
    }

    /// Get the  position type of the node
    ///
    /// The position type of the node determines whether the node will be positioned in-line with its siblings or independently
    fn position_type(&self, store: &'_ Self::Data) -> Option<PositionType> {
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
}