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
Vector4
 */
pub(crate) type Vector4i8 = Vector4<i8>;
pub(crate) type Vector4i16 = Vector4<i16>;
pub(crate) type Vector4i32 = Vector4<i32>;
pub(crate) type Vector4i64 = Vector4<i64>;
pub(crate) type Vector4u8 = Vector4<u8>;
pub(crate) type Vector4u16 = Vector4<u16>;
pub(crate) type Vector4u32 = Vector4<u32>;
pub(crate) type Vector4u64 = Vector4<u64>;
pub(crate) type Vector4isize = Vector4<isize>;
pub(crate) type Vector4f32 = Vector4<f32>;
pub(crate) type Vector4f64 = Vector4<f64>;

#[derive(Debug, Copy, Clone)]
pub struct Vector4<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector4<T> {
    pub fn dot(self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }
}
impl<T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Copy> Vector4<T> where
    f64: From<T>
{
    pub unsafe fn normalise(self) -> Vector4<f64> {
        // let converted_vector: Vector4<f64> = Vector4::new(self.x as f64, self.y as f64, self.z as f64);
        let converted_vector: Vector4<f64> = Vector4::new(f64::from(self.x), f64::from(self.y), f64::from(self.z), f64::from(self.w));

        let length = ((converted_vector.x*converted_vector.x) + (converted_vector.y*converted_vector.y) + (converted_vector.z*converted_vector.z) + (converted_vector.w*converted_vector.w)).sqrt();

        Vector4::new(converted_vector.x/length, converted_vector.y/length, converted_vector.z/length, converted_vector.w/length)

    }
}
impl<T: Add<Output = T>> Add for Vector4<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}
impl<T: Sub<Output = T>> Sub for Vector4<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}
impl<T: PartialEq> PartialEq for Vector4<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y) & (self.z == other.z) & (self.w == other.w)
    }
}
impl<T: PartialEq> Eq for Vector4<T> {}

impl<T: Neg<Output = T>> Neg for Vector4<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector4_build() {
        let a = Vector4::new(1.0, 2.0, 3.0, 0.0);
        assert_eq!(a, Vector4 {x: 1.0, y: 2.0, z: 3.0, w: 0.0 });
    }

    #[test]
    fn vector4_negation() {
        let a: Vector4f64 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b: Vector4f64 = Vector4::new(-1.0, -2.0, -3.0, -4.0);

        assert_eq!(-a, b);
    }

    #[test]
    fn vector4_dot() {
        let a: Vector4f64 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b: Vector4f64 = Vector4::new(-1.0, -2.0, -3.0, -4.0);

        assert_eq!(a.dot(&b), -1.0 + -4.0 + -9.0 + -16.0);
        unsafe {
            let a = a.normalise();
            println!("{}, {}, {}, {}", a.x, a.y, a.z, a.w);
        }
    }

    #[test]
    fn vector4_normalise() {
        // let a = Vector4::new(1.0, 2.0, 3.0);
        // let b = Vector4::new(-1.0, -2.0, -3.0);
        // let a_norm = unsafe { a.normalise() };
        //
        // assert_eq!(mat)
    }

    #[test]
    fn vector4_add() {
        let a: Vector4f64 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b: Vector4f64 = Vector4::new(-1.0, -2.0, -3.0, -4.0);

        assert_eq!(a + b, Vector4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn vector4_sub() {
        let a: Vector4f64 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b: Vector4f64 = Vector4::new(-1.0, -2.0, -3.0, -4.0);

        assert_eq!(a - b, Vector4::new(2.0, 4.0, 6.0, 8.0));
    }
}