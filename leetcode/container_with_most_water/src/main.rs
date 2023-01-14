// You are given an integer array height of length n. There are n vertical
// lines drawn such that the two endpoints of the ith line are (i, 0) and
// (i, height[i]). Find two lines that together with the x-axis form a container,
// such that the container contains the most water.
// Return the maximum amount of water a container can store.
use std::cmp;

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut area, mut l, mut r) = (0, 0, height.len() - 1);

        while l < r {
            let h = cmp::min(height[l], height[r]) as usize;
            area = cmp::max(area, (r - l) * h);

            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        area as i32
    }
    // My original solution
    // pub fn max_area(height: Vec<i32>) -> i32 {
    //     // lets first create two pointers
    //     let mut left = 0;
    //     let mut right = height.len() - 1;
    //     let mut max_area = 0;
    //     // This will give us time complexity of O(n)
    //     while left < right {
    //         if height[left] > height[right] {
    //             max_area = if max_area < height[right] * (right - left) as i32 {
    //                 height[right] * (right - left) as i32
    //             } else {
    //                 max_area
    //             };
    //             right -= 1;
    //         } else {
    //             max_area = if max_area < height[left] * (right - left) as i32 {
    //                 height[left] * (right - left) as i32
    //             } else {
    //                 max_area
    //             };
    //             left += 1;
    //         }
    //     }
    //     max_area
    // }
}

fn main() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    Solution::max_area(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(49, Solution::max_area(input));
    }

    #[test]
    fn case2() {
        let input = vec![1, 1];
        assert_eq!(1, Solution::max_area(input));
    }
}
