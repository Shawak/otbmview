use image::{Rgba};
use point::Point;

pub type Image = image::RgbaImage;

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

#[derive(PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_pixel(pixel: &mut Rgba<u8>) -> Color {
        Color {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
            a: pixel[3]
        }
    }

    pub fn data(&mut self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

const COLOR_ALPHA: Color = Color { r: 0, g: 0, b: 0, a: 0 };
const COLOR_WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
const COLOR_RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
const COLOR_GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
const COLOR_BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
const COLOR_YELLOW: Color = Color { r: 255, g: 255, b: 0, a: 255 };

pub const MASK_COLORS: &'static[Color] = &[COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_YELLOW];

pub trait ImageFunctions {
    fn mask(&mut self, color: &Color);
    fn blit(&mut self, pos: Point, image: &Image);
}

impl ImageFunctions for Image {

    fn mask(&mut self, masked_color: &Color) {
        for (x, y, pixel) in self.enumerate_pixels_mut() {
            let pixel_color = Color::from_pixel(pixel);
            let mut write_color = if pixel_color == *masked_color {
                COLOR_WHITE
            } else {
                COLOR_ALPHA
            };

            pixel.data = write_color.data();
        }
    }

    // https://github.com/edubart/otclient/blob/1addf3e1766ca3fe43bdf1114c0655a971123291/src/framework/graphics/image.cpp#L100
    fn blit(&mut self, dest: Point, other: &Image) {
        for (x, y, pixel) in other.enumerate_pixels() {
            let xx = dest.x as u32 + x;
            let yy = dest.y as u32 + y;
            //if pixel.data[3] != 0 { // TODO: figure out why alpha is 0 (spr parse set's it to zero, but why does this work on otclient? what do we miss?)
                self.put_pixel(xx, yy, *pixel);
            //}
        }

    }

}