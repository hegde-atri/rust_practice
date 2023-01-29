//

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // if list 1 is empty return list 2 and vice versa
        match (list1, list2) {
            (Some(list), None) => return Some(list),
            (None, Some(list)) => return Some(list),
            (None, None) => return None,
            (Some(left), Some(right)) => {
                if left.val <= right.val {
                    return Some(Box::new(ListNode {
                        val: left.val,
                        next: Solution::merge_two_lists(left.next, Some(right)),
                    }));
                } else {
                    return Some(Box::new(ListNode {
                        val: right.val,
                        next: Solution::merge_two_lists(Some(left), right.next),
                    }));
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
    fn case1() {}
}
