#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
extern crate rayon;
extern crate image;

mod otbm;
mod spr;
mod mem_read;
mod mem_type;

fn main() {
    //otbm::read_otbm("map2.otbm".to_string()).unwrap();

    let r = spr::parse("Tibia.spr".to_string());
    match r {
        Ok(r) => println!("{}", r.version),
        Err(e) => panic!("{}", e)
    }

    println!("done");
}