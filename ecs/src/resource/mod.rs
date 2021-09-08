
use std::collections::{HashMap};


pub mod font;
pub use font::*;

pub mod layer;
pub use layer::*;

use crate::{Canvas, state::Layer};

#[derive(Default)]
pub struct ResourceManager {
    fonts: HashMap<String, FontResource>,
    pub images: LayersResourceManager,
}