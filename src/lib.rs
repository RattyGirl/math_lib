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
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}
#[allow(dead_code)]
impl Vector3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub unsafe fn normalise(self) -> Self {
        let length: f32 = ((self.x*self.x) + (self.y*self.y) + (self.z*self.z)).sqrt();
        Self {
            x: self.x/length,
            y: self.y/length,
            z: self.z/length
        }
    }
}
impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }
}
impl Eq for Vector3 {}

impl Neg for Vector3 {
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
    fn vector3_negation() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(-1.0, -2.0, -3.0);

        assert_eq!(-a, b);
    }

    #[test]
    fn vector3_dot() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(-1.0, -2.0, -3.0);

        assert_eq!(a.dot(&b), -1.0 + -4.0 + -9.0);
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
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(-1.0, -2.0, -3.0);

        assert_eq!(a + b, Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector3_sub() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(-1.0, -2.0, -3.0);

        assert_eq!(a - b, Vector3::new(2.0, 4.0, 6.0));
    }
}