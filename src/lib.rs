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
pub struct Vector3f32 {
    x: f32,
    y: f32,
    z: f32
}
#[allow(dead_code)]
impl Vector3f32 {
    fn new(x: f32, y: f32, z: f32) -> Vector3f32 {
        Vector3f32 { x, y, z }
    }

    pub fn dot(self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub unsafe fn normalise(self) -> Vector3f32 {
        let length: f32 = ((self.x*self.x) + (self.y*self.y) + (self.z*self.z)).sqrt();
        Vector3f32 {
            x: self.x/length,
            y: self.y/length,
            z: self.z/length
        }
    }
}
impl Add for Vector3f32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl Sub for Vector3f32 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
impl PartialEq for Vector3f32 {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }
}
impl Eq for Vector3f32 {}

/*
Tests
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3f32_tests() {
        let a = Vector3f32::new(1.0, 2.0, 3.0);
        let b = Vector3f32::new(16.0, 23.0, 4.0);
    }
}