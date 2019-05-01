use std::ops::*;

pub type Size = TSize<i32>;

#[derive(Copy, Clone, Debug)]
pub struct TSize<T> {
    pub width: T,
    pub height: T,
}

impl <T: Mul<Output = T> + Copy> TSize<T> {
    pub fn new(width: T, height: T) -> TSize<T> {
        TSize {
            width,
            height
        }
    }

    pub fn area(&self) -> T {
        self.width * self.height
    }
}