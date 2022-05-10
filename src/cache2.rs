


pub trait Cache {
    type Node;

    fn width(&self, node: Self::Node) -> f32;
    fn height(&self, node: Self::Node) -> f32;
    fn posx(&self, node: Self::Node) -> f32;
    fn posy(&self, node: Self::Node) -> f32;

    fn set_width(&mut self, node: Self::Node, width: f32);
    fn set_height(&mut self, node: Self::Node, height: f32);
    fn set_posx(&mut self, node: Self::Node, posx: f32);
    fn set_posy(&mut self, node: Self::Node, posy: f32);
}