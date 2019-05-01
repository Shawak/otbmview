use std::ops::*;

pub type Point = TPoint<i32>;

#[derive(Copy, Clone, Debug)]
pub struct TPoint<T> {
    pub x: T,
    pub y: T,
}

impl <T: Mul<Output = T> + Copy> TPoint<T> {
    pub fn new(x: T, y: T) -> TPoint<T> {
        TPoint {
            x,
            y
        }
    }
}

impl<T: Add<Output = T> + Copy> Add for TPoint<T> {
    type Output = TPoint<T>;

    fn add(self, other: TPoint<T>) -> TPoint<T> {
        TPoint { x: self.x + other.x, y: self.y + other.y }
    }
}