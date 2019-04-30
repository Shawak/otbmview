use std::io::{Error, Read, Seek, Cursor};
use std::mem::*;
use std::slice::*;
use std::ptr::null;

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
    fn get_str(&mut self) -> Result<String, Error>;
    fn get_str_sized(&mut self, size: usize) -> Result<String, Error>;
    fn skip(&mut self, count: u64);
}

impl<T: Read> MemRead for T {
    fn get<U>(&mut self) -> Result<U, Error> {
        unsafe {
            let mut x: U = uninitialized();
            let slice = from_raw_parts_mut(&mut x as *mut U as *mut u8, size_of::<U>());
            self.read_exact(slice).map(|_| x)
        }
    }

    fn get_str(&mut self) -> Result<String, Error> {
        let size = self.get::<u16>()?;
        self::MemRead::get_str_sized(self, size as _)
    }

    fn get_str_sized(&mut self, size: usize) -> Result<String, Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size);
        self.take(size as u64)
            .read_to_end(&mut buffer)
            .map(|_| String::from_utf8_lossy(&buffer).into())
    }

    fn skip(&mut self, count: u64) {
        std::io::copy(&mut self.take(count), &mut std::io::sink()).expect("could not skip bytes");
    }
}
