use std::io::{Error, Read, BufRead};
use std::mem::*;
use std::slice::*;

use mem_type::*;

use std::str::FromStr;

pub trait MemRead {
    fn get<T>(&mut self) -> Result<T, Error>;
    fn gets(&mut self) -> Result<String, Error>;
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

    fn gets(&mut self) -> Result<String, Error> {
        let size = self.get::<u16>()?;
        let mut buffer: Vec<u8> = Vec::new();
        self.take(size as u64).read_to_end(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

}