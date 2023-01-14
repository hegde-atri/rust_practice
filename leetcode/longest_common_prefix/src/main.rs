// TODO: find better solution

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common = strs[0].clone();
        for str in strs.iter() {
            while !str.starts_with(&common) {
                common = common.chars().take(common.len() - 1).collect();
            }
        }
        common
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!("", Solution::longest_common_prefix(strs));
    }

    #[test]
    fn case2() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!("fl", Solution::longest_common_prefix(strs));
    }
}
