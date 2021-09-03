use crate::{State, AsEntity}; 
pub use morphorm::{LayoutType, PositionType, Units, GeometryChanged, Cache};

/// Trait for setting properties on entities
pub trait PropSet: AsEntity + Sized {
    
    // LAYOUT PROPERTIES

    /// Set the desired layout type
    fn set_layout_type(self, state: &mut State, value: LayoutType) -> Self {
        state.style.layout_type.insert(self.entity(), value);

        self
    }

    /// Set the desired position type
    fn set_position_type(self, state: &mut State, value: PositionType) -> Self {
        state.style.position_type.insert(self.entity(), value);

        self
    }

    /// Set the desired width
    fn set_width(self, state: &mut State, value: Units) -> Self {
        state.style.width.insert(self.entity(), value);

        self
    }

    /// Set the desired height
    fn set_height(self, state: &mut State, value: Units) -> Self {
        state.style.height.insert(self.entity(), value);

        self
    }

    /// Set the desired left space
    fn set_left(self, state: &mut State, value: Units) -> Self {
        state.style.left.insert(self.entity(), value);

        self
    }

    /// Set the desired right space
    fn set_right(self, state: &mut State, value: Units) -> Self {
        state.style.right.insert(self.entity(), value);

        self
    }

    /// Set the desired top space
    fn set_top(self, state: &mut State, value: Units) -> Self {
        state.style.top.insert(self.entity(), value);

        self
    }

    /// Set the desired bottom space
    fn set_bottom(self, state: &mut State, value: Units) -> Self {
        state.style.bottom.insert(self.entity(), value);

        self
    }

    fn set_space(self, state: &mut State, value: Units) -> Self {
        state.style.left.insert(self.entity(), value);
        state.style.right.insert(self.entity(), value);
        state.style.top.insert(self.entity(), value);
        state.style.bottom.insert(self.entity(), value);

        self
    }

    /// Set the desired child_left space
    fn set_child_left(self, state: &mut State, value: Units) -> Self {
        state.style.child_left.insert(self.entity(), value);

        self
    }

    /// Set the desired child_right space
    fn set_child_right(self, state: &mut State, value: Units) -> Self {
        state.style.child_right.insert(self.entity(), value);

        self
    }

    /// Set the desired child_top space
    fn set_child_top(self, state: &mut State, value: Units) -> Self {
        state.style.child_top.insert(self.entity(), value);

        self
    }

    /// Set the desired child_bottom space
    fn set_child_bottom(self, state: &mut State, value: Units) -> Self  {
        state.style.child_bottom.insert(self.entity(), value);

        self
    }

    /// Set the desired child space
    fn set_child_space(self, state: &mut State, value: Units) -> Self {
        state.style.child_left.insert(self.entity(), value);
        state.style.child_right.insert(self.entity(), value);
        state.style.child_top.insert(self.entity(), value);
        state.style.child_bottom.insert(self.entity(), value);

        self
    }

    /// Set the desired space between rows
    fn set_row_between(self, state: &mut State, value: Units) -> Self {
        state.style.row_between.insert(self.entity(), value);

        self
    }

    /// Set the desired space between columns
    fn set_col_between(self, state: &mut State, value: Units) -> Self {
        state.style.col_between.insert(self.entity(), value);

        self
    }

    /// Set the desired grid rows
    fn set_grid_rows(self, state: &mut State, value: Vec<Units>) -> Self {
        state.style.grid_rows.insert(self.entity(), value);

        self
    }

    /// Set the desired grid columns
    fn set_grid_cols(self, state: &mut State, value: Vec<Units>) -> Self {
        state.style.grid_cols.insert(self.entity(), value);

        self
    }

    /// Set the desired grid row index
    fn set_row(self, state: &mut State, index: usize, span: usize) -> Self {
        state.style.row_index.insert(self.entity(), index);
        state.style.row_span.insert(self.entity(), span);

        self
    }

    /// Set the desired grid row span
    fn set_col(self, state: &mut State, index: usize, span: usize) -> Self {
        state.style.col_index.insert(self.entity(), index);
        state.style.col_span.insert(self.entity(), span);

        self
    }

    fn set_min_width(self, state: &mut State, value: Units) -> Self {
        state.style.min_width.insert(self.entity(), value);

        self
    }

    fn set_min_height(self, state: &mut State, value: Units) -> Self {
        state.style.min_height.insert(self.entity(), value);

        self
    }
}


impl<T: AsEntity> PropSet for T {}