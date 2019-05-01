// https://github.com/edubart/otclient/blob/e870110875627d55006236b7e4996f28fed9a287/src/client/spritemanager.cpp

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};

use mem_read::*;

use image::{ImageBuffer};
use rayon::prelude::*;

use draw::*;

const SPRITE_DATA_SIZE: u32 = 32 * 32 * 4;

pub struct SpriteData {
    pub version: u32,
    pub sprites: HashMap<u32, Image>,
}

impl SpriteData {
    pub fn get_image(&self, id: u32) -> Option<&Image> {
        self.sprites.get(&id)
    }
}

pub fn parse(filename: String) -> Result<SpriteData, Error> {
    let mut file = File::open(filename)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let data: &mut &[u8] = &mut data.as_ref();
    let begin = data.clone();

    let version = data.get()?;

    let count = data.get::<u32>()?;
    let mut vec = Vec::with_capacity(count as _);
    vec.extend((0..count).flat_map(|id| {
        data.get::<u32>()
            .ok()
            .and_then(|x| if x == 0 { None } else { Some((id + 1, x)) })
    }));

    Ok(SpriteData {
        version,
        sprites: vec
            .into_par_iter()
            .map(|n| {
                let data: &mut &[u8] = &mut &begin[n.1 as _..];
                let mut img: Image = ImageBuffer::new(32, 32);

                let color_key = data.get::<[u8; 3]>()?;
                let size = data.get()?;

                let mut write = 0;
                let mut read = 0;
                let mut i = 0;
                while read < size && write < SPRITE_DATA_SIZE {
                    let transparent_count = data.get::<u16>()?;
                    let colored_count = data.get::<u16>()?;

                    for _ in 0..transparent_count {
                        img.get_pixel_mut(i / 32, i % 32).data = [0, 0, 0, 0];
                        i += 1;
                    }

                    for _ in 0..colored_count {
                        let (r, g, b, a) = (data.get()?, data.get()?, data.get()?, 0); // TODO: , 0)?
                        img.get_pixel_mut(i / 32, i % 32).data = [r, g, b, a];
                        i += 1;
                    }

                    write += 4 * transparent_count as u32 + 4 * colored_count as u32;
                    read += 4 + (3 /* channels */ * colored_count);
                }

                //img.save(format!("sprites/{}.png", n.0))?;

                Ok((n.0, img))
            })
            .collect::<Result<_, Error>>()?,
    })
}
