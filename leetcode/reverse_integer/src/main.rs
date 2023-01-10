// Given a signed 32-bit integer x, return x with its digits reversed.
// If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1],
// then return 0. Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

use std::ops::Neg;

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x.is_negative() {
            (x * -1)
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse()
                .unwrap_or(0)
                .neg()
        } else {
            x.to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse()
                .unwrap_or(0)
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(123 as i32, Solution::reverse(321));
    }

    #[test]
    fn case2() {
        assert_eq!(-321 as i32, Solution::reverse(-123));
    }
}
