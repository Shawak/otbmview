use std;
use std::fs::File;
use std::io::{Error, Cursor, Read, Write, Seek, SeekFrom};

use mem_read::*;
use mem_type::*;

use rayon::prelude::*;
use image::{ImageBuffer, *};

pub struct SpriteData {
    pub version: u32,
    pub sprites: Vec<Option<Sprite>>
}

pub struct Sprite {
    color_key: [u8; 3],
    size: u16,
    pixels: Vec<Pixel>
}

enum Pixel {
    Transparent,
    RGBA(u8, u8, u8, u8)
}

pub fn parse(filename: String) -> Result<SpriteData, Error> {
    let mut file = File::open(filename)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let mut data = Cursor::new(data);

    let mut sprites: Vec<Option<Sprite>> = Vec::new();
    let sprite_data = SpriteData {
        version: data.get()?,
        sprites
    };

    let sprite_count = data.get::<u32>()?;
    println!("Sprite Count: {}", sprite_count);
    println!("SPR Version: {}", sprite_data.version);

    for i in 1..sprite_count+1 {
        let addr = data.get::<u32>()?;
        if addr == 0 {
            continue;
        }

        let before = data.position();
        data.seek(SeekFrom::Start(addr as u64))?;
        let mut sprite = Sprite {
            color_key: data.get::<[u8; 3]>()?,
            size: data.get()?,
            pixels: Vec::new()
        };

        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(32, 32);

        const SPRITE_DATA_SIZE: u32 = 32 * 32 * 4;

        let mut write = 0;
        let mut read = 0;

        let mut i: u32 = 0;
        while read < sprite.size && write < SPRITE_DATA_SIZE  {
            let transparent_count = data.get::<u16>()?;
            let colored_count = data.get::<u16>()?;

            for _ in 0..transparent_count {
                sprite.pixels.push(Pixel::Transparent);
                img.get_pixel_mut(i/32, i%32).data = [0, 0, 0, 0];
                i+=1;
            }

            for _ in 0..colored_count {
                let (r, g, b, a) = (data.get()?, data.get()?, data.get()?, 0);
                sprite.pixels.push(Pixel::RGBA(r, g, b, a));

                img.get_pixel_mut(i/32, i%32).data = [r, g, b, a];
                i+=1;
            }

            write += 4 * transparent_count as u32 + 4 * colored_count as u32;
            read += 4 + (3 * colored_count);
        }

        //img.save(format!("test/{}.png", i))?;
        data.seek(SeekFrom::Start(before as u64))?;
    }

    Ok(sprite_data)
}