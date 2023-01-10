// string to int

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        s.trim()
            .split_whitespace()
            .next()
            .unwrap_or("0")
            .parse()
            .unwrap_or(0)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1324,
            Solution::my_atoi(String::from("   +1324 extra words"))
        );
    }
}
