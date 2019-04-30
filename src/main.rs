#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

#[macro_use]
extern crate enum_primitive_derive;
extern crate image;
extern crate num_traits;
extern crate rand;
extern crate rayon;

mod dat;
mod mem_read;
mod otb;
mod otbm;
mod spr;
mod size;
mod point;
mod draw;

use std::io::{stdout, Error, Write};

use image::{GenericImage, ImageBuffer, Rgba};

use draw::*;
use dat::ThingCategory;

fn main() -> Result<(), Error> {

    // -------------
    print!("Loading otb.. ");
    stdout().flush()?;

    let otb = otb::parse("items.otb".to_string())?;
    println!("done");

    print!("Loading otbm.. ");
    stdout().flush()?;

    let otbm = otbm::parse("map2.otbm".to_string())?;
    println!("done");

    print!("Loading spr.. ");
    stdout().flush()?;

    let spr = spr::parse("Tibia.spr".to_string())?;
    println!("done");

    print!("Loading dat.. ");
    stdout().flush()?;

    let dat = dat::parse("Tibia.dat".to_string())?;
    println!("done");

    //let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(32000 * 32, 32000 * 32);

    /*for (id, thing) in &dat[&ThingCategory::Item] {
        thing.get_texture(&spr).save(format!("test/{}.png", id))?;
    }*/

    //dat[&ThingCategory::Item][&486].get_texture(&spr).save(format!("test/{}.png", 486))?;

    /*dat[&ThingCategory::Item][&103].get_texture(&spr).save(format!("test/{}.png", 103))?;
    spr.get_image(537).save(format!("test/{}-img.png", 537))?;*/

    println!("done!");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;

    Ok(())
}