/*#![feature(test)]

pub mod mem_read;
pub mod mem_type;

extern crate test;

#[cfg(test)]
mod tests {

    use test;

    use std::io::*;
    use std::fs::*;
    use test::{Bencher};
    use std::io::Error;

    use mem_read::*;

    #[bench]
    fn bench_read_one_by_one_100(b: &mut Bencher) {
        b.iter(|| -> Result<()> {
            for n in 1..100 {
                let mut file = File::open("map.otbm")?;
                let mut buffer = [0u8; 1];
                while let Ok(byte) = file.get::<u8>() {

                }
            }
            Ok(())
        });
    }

    #[bench]
    fn bench_read_to_end_100(b: &mut Bencher) {
        b.iter(|| -> Result<()> {
            for n in 1..100 {
                let mut file = File::open("map.otbm")?;
                let mut vec: Vec<u8> = Vec::new();
                file.read_to_end(&mut vec);
                let mut cursor = Cursor::new(vec);
                while let Ok(byte) = cursor.get::<u8>() {

                }
            }
            Ok(())
        });
    }

    #[bench]
    fn bench_draw_100(b: &mut Bencher) {
        b.iter(|| -> Result<()> {
            for n in 1..100 {
                let mut main: Image = ImageBuffer::new(64, 64);
                for pixel in main.pixels_mut() {
                    *pixel = Rgba([255u8, 0u8, 0u8, 255u8]);
                }

                let mut img: Image = ImageBuffer::new(32, 32);
                for pixel in img.pixels_mut() {
                    *pixel = Rgba([0u8, 0u8, 0u8, 255u8]);
                }

                main.draw(0, 0, &mut img);
            }
            Ok(())
        });
    }

}*/
