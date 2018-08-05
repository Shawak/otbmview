use std::io::{Error, Read};
use std::mem::*;
use std::slice::*;

pub trait MemType {}

impl MemType for u8 {}
impl MemType for u16 {}
impl MemType for u32 {}
impl MemType for u64 {}

impl MemType for i8 {}
impl MemType for i16 {}
impl MemType for i32 {}
impl MemType for i64 {}

pub trait MemRead {
    fn get<U>(&mut self) -> Result<U, Error>;

    fn gets(&mut self) -> Result<String, Error>;
}

impl<T: Read> MemRead for T {
    fn get<U>(&mut self) -> Result<U, Error> {
        unsafe {
            let mut x: U = uninitialized();
            let slice = from_raw_parts_mut(&mut x as *mut U as *mut u8, size_of::<U>());
            self.read_exact(slice).map(|_| x)
        }
    }

    fn gets(&mut self) -> Result<String, Error> {
        let size = self.get::<u16>()?;
        let mut buffer: Vec<u8> = Vec::new();
        self.take(size as u64)
            .read_to_end(&mut buffer)
            .map(|_| String::from_utf8_lossy(&buffer).into())
    }
}
