extern crate image;

use image::{GenericImageView, DynamicImage};

pub fn load_image(filepath: &str) -> DynamicImage {
    let loaded_image: DynamicImage = image::open(filepath).unwrap();
    return loaded_image; 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_image() {
        //tests are run from root src directory
        const TEST_FILE: &str =  "src/images/dog.png";
        let loaded_image: DynamicImage = load_image(TEST_FILE);
        let (width, height) = loaded_image.dimensions();

        assert!(width > 0 && height > 0);
    }
}