#![allow(dead_code)]

use std::iter::successors;

use common_lib::ListNode;

fn main() {
    let head = ListNode::from(&[5, 4, 2, 1]);

    println!("Result -> {}", pair_sum(head));
}

/// - Patterns: linkedlist
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
// NOTE: There is more efficient way using fast and slow, but I prefer this one. :)
fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let list: Vec<i32> = successors(head.as_ref(), |n| n.next.as_ref())
        .map(|n| n.val)
        .collect();

    let mut left = 0;
    let mut right = list.len() - 1;

    let mut max_value = i32::MIN;
    while left < right {
        max_value = max_value.max(list[left] + list[right]);

        left += 1;
        right -= 1;
    }

    max_value
}
