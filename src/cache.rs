pub trait Cache {
    type Node;

    /// Returns the cached width of the given node.
    fn width(&self, node: Self::Node) -> f32;
    /// Returns the cached height of the given node.
    fn height(&self, node: Self::Node) -> f32;
    /// Returns the cached horizontal position of the given node.
    fn posx(&self, node: Self::Node) -> f32;
    /// Returns the cached vertical position of the given node.
    fn posy(&self, node: Self::Node) -> f32;

    /// Sets the cached width of the given node.
    fn set_width(&mut self, node: Self::Node, width: f32);
    /// Sets the cached height of the given node.
    fn set_height(&mut self, node: Self::Node, height: f32);
    /// Sets the cached horizontal position of the given node.
    fn set_posx(&mut self, node: Self::Node, posx: f32);
    /// Sets the cached vertical position of the given node.
    fn set_posy(&mut self, node: Self::Node, posy: f32);
}
