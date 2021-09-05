pub mod font;
use std::collections::{HashMap, VecDeque};

pub use font::*;

#[derive(Default)]
pub struct ResourceManager {
    fonts: HashMap<String, FontResource>,
}



// pub struct LayersResourceManager {
//     free_layers: VecDeque<Layer>,
// }

// impl LayersResourceManager {
    // pub fn new() -> Self {
    //     Self {

    //     }
    // }

    // Creates a new layer resource, re-using one if available
    // pub fn create() -> Layer {

    // }


// }