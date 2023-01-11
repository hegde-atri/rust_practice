// simple match case adding and removing from stack
// based on my java solution

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            } else if match stack.last() {
                Some('(') => c == ')',
                Some('[') => c == ']',
                Some('{') => c == '}',
                _ => false,
            } {
                stack.pop();
            } else {
                return false;
            }
        }
        stack.is_empty()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::is_valid(String::from("()[]{}")));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::is_valid(String::from("(]")));
    }
}
