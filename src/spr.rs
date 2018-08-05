// https://github.com/edubart/otclient/blob/e870110875627d55006236b7e4996f28fed9a287/src/client/spritemanager.cpp

use std;
use std::fs::File;
use std::io::{Error, Cursor, Read, Write, Seek, SeekFrom};
use std::collections::HashMap;

use mem_read::*;
use mem_type::*;

use rayon::prelude::*;
use image::{ImageBuffer, *};

use std::sync::*;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

const SPRITE_DATA_SIZE: u32 = 32 * 32 * 4;

pub struct SpriteData {
    pub version: u32,
    pub sprites: HashMap<u32, Image>,
}

pub fn parse(filename: String) -> Result<SpriteData, Error> {
    let mut file = File::open(filename)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let mut data: &[u8] = data.as_ref();

    let version = data.get()?;
    Ok(SpriteData {
        version,
        sprites: (0..data.get::<u32>()?)
            .flat_map(|id| {
                data.get::<u32>()
                    .ok()
                    .and_then(|x| if x == 0 { None } else { Some((id + 1, x)) })
            }).par_iter()
            .map(|n| (n.0, ImageBuffer::new(32, 32)))
            .collect::<HashMap<_, _>>(),
    })
}

/*pub fn parse(filename: String) -> Result<SpriteData, Error> {
    let mut file = File::open(filename)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let mut data = Cursor::new(data);

    let mut spr = SpriteData {
        version: data.get()?,
        sprites: HashMap::new(),
    };

    // multi thread
    let sprite_count = data.get::<u32>()?;
    let mut lookup: Vec<(u32, u32)> = Vec::with_capacity(sprite_count as usize);
    let arc = Arc::new(Mutex::new(spr));

    for id in 1..sprite_count + 1 {
        let addr = data.get::<u32>()?;
        if addr == 0 {
            continue;
        }
        lookup.push((id, addr));
    }

    (1..10).collect();

    lookup.par_iter().for_each(|n| {
        let mut img: Image = ImageBuffer::new(32, 32);

        arc.lock().unwrap().sprites.insert(n.0, img);
        //spr.sprites.insert(n.0, img);
    });


    // single thread
    /*let sprite_count = data.get::<u32>()?;
    for k in 1..sprite_count + 1 {
        let addr = data.get::<u32>()?;
        if addr == 0 {
            continue;
        }

        let mut img: Image = ImageBuffer::new(32, 32);

        let before = data.position();
        data.seek(SeekFrom::Start(addr as u64))?;

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
                let (r, g, b, a) = (data.get()?, data.get()?, data.get()?, 0);
                img.get_pixel_mut(i / 32, i % 32).data = [r, g, b, a];
                i += 1;
            }

            write += 4 * transparent_count as u32 + 4 * colored_count as u32;
            read += 4 + (3 /* channels */ * colored_count);
        }

        // img.save(format!("test/{}.png", k))?;
        spr.sprites.insert(k, img);
        data.seek(SeekFrom::Start(before as u64))?;
    }*/

    Ok(arc.to_owned().to_owned().get_mut().)
}*/