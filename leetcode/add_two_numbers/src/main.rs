// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order, and each of their nodes contains a single digit.
// Add the two numbers and return the sum as a linked list. You may assume the two numbers
// do not contain any leading zero, except the number 0 itself.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let n1 = Self::parse_list(l1.unwrap()).unwrap();
        let n2 = Self::parse_list(l2.unwrap()).unwrap();
        let sum = n1 + n2;
        return Self::convert_to_node(sum);
    }

    fn parse_list(list: Box<ListNode>) -> Result<i32, String> {
        let mut next = true;
        let mut nums = vec![list.val];
        let mut previous = &list;
        while next {
            if let Some(ref node) = previous.next {
                let current: &Box<ListNode> = node;
                nums.push(current.val);
                previous = current;
            } else {
                next = false;
            }
        }
        if let Ok(val) = Self::convert_to_num(nums) {
            return Ok(val);
        } else {
            Err(String::from("Could not convert num vec to string"))
        }
    }

    fn convert_to_num(nums: Vec<i32>) -> Result<i32, String> {
        return Ok(nums.iter().rev().fold(0, |acc, elem| acc * 10 + elem) as i32);
    }

    fn convert_to_node(num: i32) -> Option<Box<ListNode>> {}
}

fn main() {}
