// string to int

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let out = s
            .trim()
            .chars()
            .enumerate()
            .take_while(|&(i, c)| match c {
                '0'..='9' => true,
                '+' | '-' => i == 0 as usize,
                _ => false,
            })
            .map(|(_, c)| c)
            .collect::<String>();

        if out.is_empty() {
            return 0;
        }

        match out.parse::<i32>() {
            Ok(a) => a,
            Err(_) => {
                let signs = out.chars().take_while(|&c| c == '+' || c == '-').count();
                if signs > 1 {
                    0
                } else if signs == 1 && out.len() == 1 {
                    0
                } else if out.trim().starts_with("-") {
                    return i32::MIN;
                } else {
                    return i32::MAX;
                }
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::my_atoi(String::from(" +-")));
    }

    #[test]
    fn case2() {
        assert_eq!(
            2147483647,
            Solution::my_atoi(String::from("20000000000000000000"))
        );
    }
}
