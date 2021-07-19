use crate::Node;

/// The Cache stores the result of layout as well as intermediate values for each node
pub trait Cache {
    type Item: Node;

    /// Reset all cache values to default
    fn reset(&mut self);

    /// Get the computed width of a node
    fn width(&self, node: &Self::Item) -> f32;

    /// Get the computed height of a node
    fn height(&self, node: &Self::Item) -> f32;

    /// Get the computed x position of a node
    fn posx(&self, node: &Self::Item) -> f32;

    /// Get the computed y position of a node
    fn posy(&self, node: &Self::Item) -> f32;

    fn left(&self, node: &Self::Item) -> f32;
    fn right(&self, node: &Self::Item) -> f32;
    fn top(&self, node: &Self::Item) -> f32;
    fn bottom(&self, node: &Self::Item) -> f32;

    /// Get the computed maximum width of the child nodes
    fn child_width_max(&self, node: &Self::Item) -> f32;

    /// Get the computed sum of the widths of the child nodes
    fn child_width_sum(&self, node: &Self::Item) -> f32;

    /// Get the computed maximum width of the child nodes
    fn child_height_max(&self, node: &Self::Item) -> f32;

    /// Get the computed sum of the widths of the child nodes
    fn child_height_sum(&self, node: &Self::Item) -> f32;

    /// Get the computed maximum grid row
    fn grid_row_max(&self, node: &Self::Item) -> f32;

    /// Get the computed maximum grid column
    fn grid_col_max(&self, node: &Self::Item) -> f32;

    // Setters
    fn set_child_width_sum(&mut self, node: &Self::Item, value: f32);
    fn set_child_height_sum(&mut self, node: &Self::Item, value: f32);
    fn set_child_width_max(&mut self, node: &Self::Item, value: f32);
    fn set_child_height_max(&mut self, node: &Self::Item, value: f32);

    fn horizontal_free_space(&self, node: &Self::Item) -> f32;
    fn set_horizontal_free_space(&mut self, node: &Self::Item, value: f32);
    fn vertical_free_space(&self, node: &Self::Item) -> f32;
    fn set_vertical_free_space(&mut self, node: &Self::Item, value: f32);

    fn horizontal_stretch_sum(&self, node: &Self::Item) -> f32;
    fn set_horizontal_stretch_sum(&mut self, node: &Self::Item, value: f32);
    fn vertical_stretch_sum(&self, node: &Self::Item) -> f32;
    fn set_vertical_stretch_sum(&mut self, node: &Self::Item, value: f32);

    fn set_width(&mut self, node: &Self::Item, value: f32);
    fn set_height(&mut self, node: &Self::Item, value: f32);
    fn set_posx(&mut self, node: &Self::Item, value: f32);
    fn set_posy(&mut self, node: &Self::Item, value: f32);

    fn set_left(&mut self, node: &Self::Item, value: f32);
    fn set_right(&mut self, node: &Self::Item, value: f32);
    fn set_top(&mut self, node: &Self::Item, value: f32);
    fn set_bottom(&mut self, node: &Self::Item, value: f32);

    fn stack_first_child(&self, node: &Self::Item) -> bool;
    fn set_stack_first_child(&mut self, node: &Self::Item, value: bool);
    fn stack_last_child(&self, node: &Self::Item) -> bool;
    fn set_stack_last_child(&mut self, node: &Self::Item, value: bool);
}
