// Given a string s, find the length of the longest
// substring without repeating characters.
// perhaps the title of the project is a bit misleading/vague :P

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // pointer
        let mut index = 0;
        let mut set = HashSet::new();
        let list = s.chars().collect::<Vec<char>>();
        // loop through characters
        list.iter()
            .map(|c| {
                // emulates sliding window
                while set.contains(&c) {
                    set.remove(&list[index]);
                    index += 1;
                }
                set.insert(c);
                set.len()
            })
            .max()
            .unwrap_or(0) as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("pwwkew"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("abcabcbb"))
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring(String::from("bbbbb"))
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("pwwkew"))
        );
    }
}
