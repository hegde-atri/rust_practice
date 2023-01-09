// Given two sorted arrays nums1 and nums2 of size m and n respectively,
// return the median of the two sorted arrays.
// The overall run time complexity should be O(log (m+n)).

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // join the arrays and sort them
        let mut arr: Vec<i32> = [nums1, nums2].concat();
        arr.sort_unstable();
        // find and return "mid-point"
        if arr.len() % 2 == 0 {
            return (arr[arr.len() / 2 - 1] + arr[arr.len() / 2]) as f64 / 2 as f64;
        } else {
            return arr[arr.len() / 2] as f64;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(
            2.5 as f64,
            Solution::find_median_sorted_arrays(nums1, nums2)
        );
    }
}
