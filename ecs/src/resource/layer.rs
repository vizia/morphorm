use std::collections::{BTreeMap, VecDeque};
use femtovg::{ErrorKind, ImageFlags, ImageId, PixelFormat};

use crate::{state::Layer, Canvas};


#[derive(Debug)]
pub struct ImageResource {
    pub width: usize,
    pub height: usize,
    pub image: ImageId,
}


#[derive(Default)]
pub struct LayersResourceManager {
    // List of already allocated images
    free_layers: VecDeque<ImageId>,

    layers: BTreeMap<usize, Layer>,
}

impl LayersResourceManager {
    pub fn new() -> Self {
        Self {
            free_layers: VecDeque::<ImageId>::new(),
            layers: BTreeMap::new(),
        }
    }

    // Creates a new layer resource, re-using one if available
    // pub fn create(&mut self, canvas: &mut Canvas, width: usize, height: usize) -> Result<ImageResource, ErrorKind> {
    //     if let Some(image) = self.free_layers.pop_back() {
            
    //         canvas.realloc_image(image, width, height, PixelFormat::Rgba8, ImageFlags::FLIP_Y)?;
            
    //         Ok(ImageResource {
    //             width,
    //             height,
    //             image,
    //         })

    //     } else {

    //         Ok(ImageResource {
    //             width,
    //             height,
    //             image: canvas.create_image_empty(width, height, PixelFormat::Rgba8, ImageFlags::FLIP_Y)?,
    //         })
            
    //     }
    // }

    pub fn get(&mut self, canvas: &mut Canvas, image_id: &mut Option<ImageId>, width: usize, height: usize) -> bool {
        
        if let Some(image) = image_id {
            if let Ok((w, h)) = canvas.image_size(*image) {
                if width != w || height != h {
                    canvas.delete_image(*image);
                    //canvas.realloc_image(image, width, height, PixelFormat::Rgba8, ImageFlags::FLIP_Y).expect("Failed to realloc");
                    *image_id = canvas.create_image_empty(width, height, PixelFormat::Rgba8, ImageFlags::FLIP_Y).ok();
                    return true;
                }
            }

            return false;
        } else {
            *image_id = canvas.create_image_empty(width, height, PixelFormat::Rgba8, ImageFlags::FLIP_Y).ok();
            return true;
        }
    
    }

}

pub fn next_power_of_two(input: u32) -> u32 {
    let mut v = input - 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_power_of_two() {
        let input = 400;
        let output = next_power_of_two(input);
        println!("{}", output);
    }
}