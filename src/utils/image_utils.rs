extern crate image;

use std::io:File;
use image::{GenericImage, DynamicImage, ImageBuffer, imageops}

pub fn load_image(filepath: &str) -> DynamicImage {
    let mut loaded_image = image::open(filepath).unwrap();
    return loaded_image; 
}

#[cfg(test)]
mod tests {
    use super::*

    #[test]
    fn test_load_image() {
        const test_file = "../images/dog.png"
        let mut loaded_image = load_image
    }
}