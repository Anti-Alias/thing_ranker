use std::io::Cursor;

use image::{ImageError, ImageFormat, ImageReader};
use thiserror::Error;

/// Takes raw image bytes with some encoding (PNG, JPG, GIF etc), and converts it
/// to WebP bytes.
pub fn process_image(image_bytes: &[u8]) -> Result<Vec<u8>, ImageProcessingError> {
    let image = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()?
        .decode()?;
    let mut png_bytes = Vec::<u8>::new();
    let mut png_writer = Cursor::new(&mut png_bytes);
    image.write_to(&mut png_writer, ImageFormat::WebP)?;
    Ok(png_bytes)
}

#[derive(Error, Debug)]
pub enum ImageProcessingError {
    #[error("IO error occurred when processing image: {0}")]
    IOError(#[from] std::io::Error),
    #[error("ImageError occurred when processing image: {0}")]
    ImageError(#[from] ImageError),
}
