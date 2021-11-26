use crate::Node;
use crate::types::GeometryChanged;

/// The Cache stores the result of layout as well as intermediate values for each node
pub trait Cache {
    type Item: for<'a> Node<'a>;

    // Getters

    /// Get the geometry changed bitflag of the node
    fn geometry_changed(&self, node: Self::Item) -> GeometryChanged;

    /// Get the visibility flag of the node
    fn visible(&self, node: Self::Item) -> bool;

    /// Get the computed width of a node
    fn width(&self, node: Self::Item) -> f32;

    /// Get the computed height of a node
    fn height(&self, node: Self::Item) -> f32;

    /// Get the computed x position of a node
    fn posx(&self, node: Self::Item) -> f32;

    /// Get the computed y position of a node
    fn posy(&self, node: Self::Item) -> f32;

    /// Get the computed space to the left of a node
    fn main_before(&self, node: Self::Item) -> f32;
    /// Get the computed space to the right of a node
    fn main_after(&self, node: Self::Item) -> f32;
    /// Get the computed space above a node
    fn cross_before(&self, node: Self::Item) -> f32;
    /// Get the computed space below a node
    fn cross_after(&self, node: Self::Item) -> f32;

    fn new_main(&self, node: Self::Item) -> f32;
    fn new_cross(&self, node: Self::Item) -> f32;

    /// Get the computed maximum width of the child nodes
    fn child_main_max(&self, node: Self::Item) -> f32;

    /// Get the computed sum of the widths of the child nodes
    fn child_main_sum(&self, node: Self::Item) -> f32;

    /// Get the computed maximum width of the child nodes
    fn child_cross_max(&self, node: Self::Item) -> f32;

    /// Get the computed sum of the widths of the child nodes
    fn child_cross_sum(&self, node: Self::Item) -> f32;

    /// Get the computed maximum grid row
    fn grid_row_max(&self, node: Self::Item) -> f32;

    /// Set the computed maximum grid row size
    fn set_grid_row_max(&mut self, node: Self::Item, value: f32);

    /// Get the computed maximum grid column
    fn grid_col_max(&self, node: Self::Item) -> f32;

    /// Set the computed maximum grid column size
    fn set_grid_col_max(&mut self, node: Self::Item, value: f32);

    // Setters

    fn set_visible(&mut self, node: Self::Item, value: bool);

    fn set_geo_changed(&mut self, node: Self::Item, flag: GeometryChanged, value: bool);

    fn set_child_main_sum(&mut self, node: Self::Item, value: f32);
    fn set_child_cross_sum(&mut self, node: Self::Item, value: f32);
    fn set_child_main_max(&mut self, node: Self::Item, value: f32);
    fn set_child_cross_max(&mut self, node: Self::Item, value: f32);

    fn main_free_space(&self, node: Self::Item) -> f32;
    fn set_main_free_space(&mut self, node: Self::Item, value: f32);
    fn cross_free_space(&self, node: Self::Item) -> f32;
    fn set_cross_free_space(&mut self, node: Self::Item, value: f32);

    fn main_stretch_sum(&self, node: Self::Item) -> f32;
    fn set_main_stretch_sum(&mut self, node: Self::Item, value: f32);
    fn cross_stretch_sum(&self, node: Self::Item) -> f32;
    fn set_cross_stretch_sum(&mut self, node: Self::Item, value: f32);

    fn set_width(&mut self, node: Self::Item, value: f32);
    fn set_height(&mut self, node: Self::Item, value: f32);
    fn set_posx(&mut self, node: Self::Item, value: f32);
    fn set_posy(&mut self, node: Self::Item, value: f32);

    fn set_main_before(&mut self, node: Self::Item, value: f32);
    fn set_main_after(&mut self, node: Self::Item, value: f32);
    fn set_cross_before(&mut self, node: Self::Item, value: f32);
    fn set_cross_after(&mut self, node: Self::Item, value: f32);

    fn set_new_main(&mut self, node: Self::Item, value: f32);
    fn set_new_cross(&mut self, node: Self::Item, value: f32);

    fn stack_first_child(&self, node: Self::Item) -> bool;
    fn set_stack_first_child(&mut self, node: Self::Item, value: bool);
    fn stack_last_child(&self, node: Self::Item) -> bool;
    fn set_stack_last_child(&mut self, node: Self::Item, value: bool);
}
