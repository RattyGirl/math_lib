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
#![allow(unused_variables)]
#![allow(dead_code)]


pub mod vector3;
pub mod vector2;
pub mod vector4;

/*
Static
*/
pub fn sigmoid(x: f64) -> f64 {
    let one: f64 = 1.0;
    one/(one + std::f64::consts::E.powf(-x))
}

/*
Tests
 */
#[cfg(test)]
mod tests {
    use super::*;
}