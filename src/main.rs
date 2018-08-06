#![allow(unused_variables)]

#[macro_use]
extern crate enum_primitive_derive;
extern crate image;
extern crate num_traits;
extern crate rand;
extern crate rayon;

mod dat;
mod mem_read;
mod otbm;
mod spr;
mod draw;

use std::io::{stdout, Error, Write};

use image::{GenericImage, ImageBuffer, Rgba};

use draw::*;

fn main() -> Result<(), Error> {

    println!(" done");

    let mut s = String::new();
    std::io::stdin().read_line(&mut s);

    print!("Loading otbm..");
    stdout().flush()?;

    let otbm = otbm::read_otbm("map2.otbm".to_string())?;
    println!(" done");

    print!("Loading spr..");
    stdout().flush()?;

    let spr = spr::parse("Tibia.spr".to_string())?;
    println!(" done");

    print!("Loading dat..");
    stdout().flush()?;

    let dat = dat::parse("Tibia.dat".to_string())?;
    println!(" done");

    //let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(32000 * 32, 32000 * 32);

    let mut s = String::new();
    std::io::stdin().read_line(&mut s);

    Ok(())
}
