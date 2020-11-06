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
/*
Static functions
*/
pub fn sigmoid(x: f64) -> f64 {
    let one: f64 = 1.0;
    one/(one + std::f64::consts::E.powf(-x))
}

/*
Structs
*/

/*
Member Functions
*/

/*
Tests
 */
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn this_test_will_pass() {
    //     let value = prints_and_returns_10(4);
    //     assert_eq!(10, value);
    // }
    //
    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }
}