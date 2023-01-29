// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        Self::generate(&mut res, &mut String::new(), 0, 0, n);
        return res;
    }

    fn generate(res: &mut Vec<String>, s: &mut String, l: i32, r: i32, n: i32) {
        if l == n && r == n {
            res.push(s.to_string());
        }
        if l < n {
            Solution::generate(res, &mut (s.to_owned() + "("), l + 1, r, n);
        }
        if r < l {
            Solution::generate(res, &mut (s.to_owned() + ")"), l, r + 1, n);
        }
    }
}

fn main() {
    Solution::generate_parenthesis(3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"],
            Solution::generate_parenthesis(3)
        )
    }
}
