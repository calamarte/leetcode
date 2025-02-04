#![allow(dead_code)]

use std::{cmp::Reverse, collections::BinaryHeap, iter::successors};

use leetcode::ListNode;

fn main() {
    let head = ListNode::from(&[4, 2, 1, 3]);

    println!("Result -> {:?}", sort_list(head));
}

/// # Heap-Based sorting
/// Time complexity: $$O(n \cdot log n)$$
/// Space complexity: $$O(n)$$
fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut min_heap: BinaryHeap<_> = successors(head.as_ref(), |n| n.next.as_ref())
        .map(|n| Reverse(n.val))
        .collect();

    let mut dummy = ListNode::new(-1);
    let mut current = &mut dummy;

    while let Some(Reverse(min)) = min_heap.pop() {
        current.next = Some(Box::new(ListNode::new(min)));
        current = current.next.as_mut().unwrap();
    }

    dummy.next
}
