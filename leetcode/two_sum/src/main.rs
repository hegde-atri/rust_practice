// Given an array of integers nums and an integer target, return
// indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.
// You can return the answer in any order.

use std::{collections::HashMap, time::Instant};

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // hashmap
        let mut map: HashMap<i32, i32> = HashMap::new();
        // iterate
        for (i, num) in nums.iter().enumerate() {
            let complement = target - nums[i];
            match map.get(&complement) {
                Some(n) => {
                    return vec![*n, i as i32];
                }
                None => {
                    // insert with num(key) pointing to index(value)
                    map.insert(*num, i as i32);
                }
            }
        }
        vec![]
    }
}

fn main() {
    let nums: Vec<i32> = [2, 7, 11, 15].to_vec();
    let start = Instant::now();
    let answer = Solution::two_sum(nums, 9);
    let duration = start.elapsed();
    println!("Answer: {:?}", answer);
    println!("Time taken: {:?}", duration);
}
