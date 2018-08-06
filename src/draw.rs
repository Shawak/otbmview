use image::{GenericImage, ImageBuffer, Rgba};

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub trait Drawable {
    fn draw(&mut self, x: u32, y: u32, other: &mut Image);
}

impl Drawable for Image {
    fn draw(&mut self, x_offset: u32, y_offset: u32, other: &mut Image) {
        for (x, y, pixel) in other.enumerate_pixels_mut() {
            self[(x + x_offset, y + y_offset)] = *pixel;
        }
    }
}