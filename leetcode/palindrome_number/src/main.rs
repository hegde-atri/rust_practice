// Given an integer x, return true if x is a palindrome , and false otherwise.

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Lets make a the reverse number then compare equality
        let mut rev: i32 = 0;
        let mut y = x;

        while y > 0 {
            // kinda works like a stack
            rev = rev * 10 + y % 10;
            y /= 10;
        }

        rev == x
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::is_palindrome(12321))
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::is_palindrome(123445621))
    }
}
