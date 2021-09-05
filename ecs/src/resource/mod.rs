pub mod font;
use std::collections::{HashMap, VecDeque};

use femtovg::{ErrorKind, ImageFlags, ImageId, PixelFormat};
pub use font::*;

use crate::{Canvas, state::Layer};

#[derive(Default)]
pub struct ResourceManager {
    fonts: HashMap<String, FontResource>,
    pub images: LayersResourceManager,
}



#[derive(Default)]
pub struct LayersResourceManager {
    // List of already allocated images
    free_layers: VecDeque<ImageId>,
}

impl LayersResourceManager {
    pub fn new() -> Self {
        Self {
            free_layers: VecDeque::<ImageId>::new(),
        }
    }

    // Creates a new layer resource, re-using one if available
    pub fn create(&mut self, canvas: &mut Canvas, layer: &mut Layer) -> Result<(), ErrorKind> {
        if let Some(image) = self.free_layers.pop_back() {
            canvas.realloc_image(image, layer.width as usize, layer.height as usize, PixelFormat::Rgba8, ImageFlags::empty())?;
            layer.image = Some(image);
        } else {
            layer.image = canvas.create_image_empty(layer.width as usize, layer.height as usize, PixelFormat::Rgba8, ImageFlags::empty()).ok();
        }

        Ok(())
    }


}