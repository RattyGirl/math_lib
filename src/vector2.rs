/*
 *     Math_Lib is a Rust Library that contains many functions to be used
 *     with the intention of calculations.
 *     Copyright (C) 2020 Chloe Holmes
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or any
 *     later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::ops::*;
use std::cmp::*;
use std::fmt::*;
/*
Vector2
 */
pub(crate) type Vector2i8 = Vector2<i8>;
pub(crate) type Vector2i16 = Vector2<i16>;
pub(crate) type Vector2i32 = Vector2<i32>;
pub(crate) type Vector2i64 = Vector2<i64>;
pub(crate) type Vector2u8 = Vector2<u8>;
pub(crate) type Vector2u16 = Vector2<u16>;
pub(crate) type Vector2u32 = Vector2<u32>;
pub(crate) type Vector2u64 = Vector2<u64>;
pub(crate) type Vector2isize = Vector2<isize>;
pub(crate) type Vector2f32 = Vector2<f32>;
pub(crate) type Vector2f64 = Vector2<f64>;

#[derive(Debug, Copy, Clone)]
pub struct Vector2<T>
{
    pub x: T,
    pub y: T
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector2<T> {
    pub fn dot(self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
}
impl<T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Copy> Vector2<T> where
    f64: From<T>
{
    pub unsafe fn normalise(self) -> Vector2<f64> {
        // let converted_vector: Vector2<f64> = Vector2::new(self.x as f64, self.y as f64, self.z as f64);
        let converted_vector: Vector2<f64> = Vector2::new(f64::from(self.x), f64::from(self.y));

        let length = ((converted_vector.x*converted_vector.x) + (converted_vector.y*converted_vector.y)).sqrt();

        Vector2::new(converted_vector.x/length, converted_vector.y/length)

    }
}
impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}
impl<T: PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }
}
impl<T: PartialEq> Eq for Vector2<T> {}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector2_build() {
        let a = Vector2::new(1.0, 2.0);
        assert_eq!(a, Vector2 {x: 1.0, y: 2.0 });
    }

    #[test]
    fn vector2_negation() {
        let a: Vector2f64 = Vector2::new(1.0, 2.0);
        let b: Vector2f64 = Vector2::new(-1.0, -2.0);

        assert_eq!(-a, b);
    }

    #[test]
    fn vector2_dot() {
        let a: Vector2f64 = Vector2::new(1.0, 2.0);
        let b: Vector2f64 = Vector2::new(-1.0, -2.0);

        assert_eq!(a.dot(&b), -1.0 + -4.0);
        unsafe {
            let a = a.normalise();
            println!("{}, {}", a.x, a.y);
        }
    }

    #[test]
    fn vector2_normalise() {
        // let a = Vector2::new(1.0, 2.0, 3.0);
        // let b = Vector2::new(-1.0, -2.0, -3.0);
        // let a_norm = unsafe { a.normalise() };
        //
        // assert_eq!(mat)
    }

    #[test]
    fn vector2_add() {
        let a: Vector2f64 = Vector2::new(1.0, 2.0);
        let b: Vector2f64 = Vector2::new(-1.0, -2.0);

        assert_eq!(a + b, Vector2::new(0.0, 0.0));
    }

    #[test]
    fn vector2_sub() {
        let a: Vector2f64 = Vector2::new(1.0, 2.0);
        let b: Vector2f64 = Vector2::new(-1.0, -2.0);

        assert_eq!(a - b, Vector2::new(2.0, 4.0));
    }
}