#![allow(unused)]

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
extern crate rayon;
extern crate image;
extern crate rand;

mod otbm;
mod spr;
mod dat;
mod mem_read;
mod mem_type;

use std::fs::File;
use std::io::{Error, Cursor, Read, Write, Seek, SeekFrom};

use mem_read::*;
use mem_type::*;

use image::{GenericImage, ImageBuffer, Rgba};

fn run() -> Result<(), Error> {

    print!("Loading otbm..");
    std::io::stdout().flush();
    let otbm = otbm::read_otbm("map2.otbm".to_string())?;
    println!(" done");

    print!("Loading spr..");
    std::io::stdout().flush();
    let spr = spr::parse("Tibia.spr".to_string())?;
    println!(" done");

    print!("Loading dat..");
    std::io::stdout().flush();
    let dat = dat::parse("Tibia.dat".to_string())?;
    println!(" done");




    //let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(32000 * 32, 32000 * 32);

    //let mut s = String::new();
    //std::io::stdin().read_line(&mut s);

    Ok(())
}

fn main() {
    match run() {
        Ok(r) => println!("done"),
        Err(e) => panic!("{}", e)
    }
}