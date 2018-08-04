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

fn run() -> Result<(), Error> {

    //otbm::read_otbm("map2.otbm".to_string()).unwrap();

    //let r = spr::parse("Tibia.spr".to_string())?;
    //println!("{}", r.version);

    dat::parse("Tibia.dat".to_string())?;

    Ok(())
}

fn main() {
    match run() {
        Ok(r) => println!("done"),
        Err(e) => panic!("{}", e)
    }
}