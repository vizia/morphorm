pub trait Cache {
    type CacheKey;

    /// Returns the cached width of the given node.
    fn width(&self, node: Self::CacheKey) -> f32;
    /// Returns the cached height of the given node.
    fn height(&self, node: Self::CacheKey) -> f32;
    /// Returns the cached horizontal position of the given node.
    fn posx(&self, node: Self::CacheKey) -> f32;
    /// Returns the cached vertical position of the given node.
    fn posy(&self, node: Self::CacheKey) -> f32;

    /// Sets the cached width of the given node.
    fn set_width(&mut self, node: Self::CacheKey, width: f32);
    /// Sets the cached height of the given node.
    fn set_height(&mut self, node: Self::CacheKey, height: f32);
    /// Sets the cached horizontal position of the given node.
    fn set_posx(&mut self, node: Self::CacheKey, posx: f32);
    /// Sets the cached vertical position of the given node.
    fn set_posy(&mut self, node: Self::CacheKey, posy: f32);
}
