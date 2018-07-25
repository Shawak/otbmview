use std::io::{Error, Read};
use std::mem::*;
use std::slice::*;

pub trait MemRead {
    fn get<T>(&mut self) -> Result<T, Error>;
}

impl<T: Read> MemRead for T {

    fn get<U>(&mut self) -> Result<U, Error> {
        unsafe {
            let mut x: U = uninitialized();
            let slice = from_raw_parts_mut(&mut x as *mut U as *mut u8, size_of::<U>());
            self.read_exact(slice)?;
            Ok(x)
        }
    }

}