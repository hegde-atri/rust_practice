pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            1.0
        } else if n == 1 {
            x
        } else {
            x.powf(n.into())
        }
    }
}

fn main() {
    println!("{}", Solution::my_pow(2.0, 10000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::my_pow(2.000000, 10), 1024.0000);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::my_pow(2.000000, -2), 0.2500);
    }
}
