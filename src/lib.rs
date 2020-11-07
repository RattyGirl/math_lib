/*
  Math_Lib is a Rust Library that contains many functions to be used
  with the intention of calculations.
  Copyright (C) 2020 Chloe Holmes

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or any
  later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::ops::*;
use std::cmp::*;
use std::fmt::*;
/*
Static
*/
pub fn sigmoid(x: f64) -> f64 {
    let one: f64 = 1.0;
    one/(one + std::f64::consts::E.powf(-x))
}

/*
f32 Vectors
 */
#[derive(Debug, Copy, Clone)]
pub struct Vector3<T>
{
    x: T,
    y: T,
    z: T
}

impl<T> Vector3<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector3<T> {
    pub fn dot(self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}
impl<T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Copy> Vector3<T> where
f64: From<T>
{
    pub unsafe fn normalise(self) -> Vector3<f64> {
        // let converted_vector: Vector3<f64> = Vector3::new(self.x as f64, self.y as f64, self.z as f64);
        let converted_vector: Vector3<f64> = Vector3::new(f64::from(self.x), f64::from(self.y), f64::from(self.z));

        let length = ((converted_vector.x*converted_vector.x) + (converted_vector.y*converted_vector.y) + (converted_vector.z*converted_vector.z)).sqrt();

        Vector3::new(converted_vector.x/length, converted_vector.y/length, converted_vector.z/length)

    }
}
impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl<T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
impl<T: PartialEq> PartialEq for Vector3<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }
}
impl<T: PartialEq> Eq for Vector3<T> {}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

/*
Tests
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3_build() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(a, Vector3 {x: 1.0, y: 2.0, z: 3.0});
    }

    #[test]
    fn vector3_negation() {
        let a: Vector3<f32> = Vector3::new(1.0, 2.0, 3.0);
        let b: Vector3<f32> = Vector3::new(-1.0, -2.0, -3.0);

        assert_eq!(-a, b);
    }

    #[test]
    fn vector3_dot() {
        let a: Vector3<f32> = Vector3::new(1.0, 2.0, 3.0);
        let b: Vector3<f32> = Vector3::new(-1.0, -2.0, -3.0);

        assert_eq!(a.dot(&b), -1.0 + -4.0 + -9.0);
        unsafe {
            let a = a.normalise();
            println!("{}, {}, {}", a.x, a.y, a.z);
        }
    }

    #[test]
    fn vector3_normalise() {
        // let a = Vector3::new(1.0, 2.0, 3.0);
        // let b = Vector3::new(-1.0, -2.0, -3.0);
        // let a_norm = unsafe { a.normalise() };
        //
        // assert_eq!(mat)
    }

    #[test]
    fn vector3_add() {
        let a: Vector3<f32> = Vector3::new(1.0, 2.0, 3.0);
        let b: Vector3<f32> = Vector3::new(-1.0, -2.0, -3.0);

        assert_eq!(a + b, Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector3_sub() {
        let a: Vector3<f32> = Vector3::new(1.0, 2.0, 3.0);
        let b: Vector3<f32> = Vector3::new(-1.0, -2.0, -3.0);

        assert_eq!(a - b, Vector3::new(2.0, 4.0, 6.0));
    }
}