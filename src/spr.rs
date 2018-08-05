// https://github.com/edubart/otclient/blob/e870110875627d55006236b7e4996f28fed9a287/src/client/spritemanager.cpp

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};

use mem_read::*;

use image::{ImageBuffer, *};
use rayon::prelude::*;

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
                        let (r, g, b, a) = (data.get()?, data.get()?, data.get()?, 0);
                        img.get_pixel_mut(i / 32, i % 32).data = [r, g, b, a];
                        i += 1;
                    }

                    write += 4 * transparent_count as u32 + 4 * colored_count as u32;
                    read += 4 + (3 /* channels */ * colored_count);
                }

                img.save(format!("test/{}.png", n.0))?;

                Ok((n.0, img))
            })
            //.collect::<HashMap<_, _>>(),
            .collect::<Result<_, Error>>()?,
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
