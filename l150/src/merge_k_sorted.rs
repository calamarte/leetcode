#![allow(dead_code)]

use std::collections::BinaryHeap;

use leetcode::{sexy_format, ListNode};

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let lists = vec![
        ListNode::from(&[1, 4, 5]),
        ListNode::from(&[1, 3, 4]),
        ListNode::from(&[2, 6]),
    ];

    // let lists = vec![None, None, None];

    println!("{}", sexy_format(merge_k_lists_heap(lists)));
}

/// Time complexity: $$O(n \cdot k)$$
/// Space complexity: $$O(n)$$
fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    while lists.len() > 1 {
        let (l1, l2) = (lists.pop().unwrap(), lists.pop().unwrap());
        let mut dummy = ListNode::new(-1);
        let mut current = &mut dummy;

        let mut l1_ref = l1.as_deref();
        let mut l2_ref = l2.as_deref();

        while l1_ref.is_some() || l2_ref.is_some() {
            match (l1_ref, l2_ref) {
                (Some(n1), Some(n2)) if n1.val <= n2.val => {
                    current.next = Some(Box::new(ListNode::new(n1.val)));
                    current = current.next.as_mut().unwrap();

                    l1_ref = n1.next.as_deref();
                }
                (Some(n1), Some(n2)) if n1.val > n2.val => {
                    current.next = Some(Box::new(ListNode::new(n2.val)));
                    current = current.next.as_mut().unwrap();

                    l2_ref = n2.next.as_deref();
                }
                (Some(n), None) | (None, Some(n)) => {
                    current.next = Some(Box::new(n.to_owned()));
                    break;
                }
                _ => panic!("Never here!"),
            }
        }

        lists.push(dummy.next);
    }

    lists.pop().unwrap_or_default()
}

#[derive(Eq, PartialEq)]
struct HeapNode(i32, ListNode);

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0) // NOTE: Prepare to min-heap avoid use Reverse()
    }
}

/// # Heap approach
/// Time complexity: $$O(n log k)$$
/// Space complexity: $$O(n)$$
fn merge_k_lists_heap(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::with_capacity(lists.len());

    for root in lists.into_iter().flatten() {
        heap.push(HeapNode(root.val, *root));
    }

    let mut dummy = ListNode::new(-1);
    let mut current = &mut dummy;

    while let Some(HeapNode(val, node)) = heap.pop() {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_deref_mut().unwrap();

        if let Some(next) = node.next {
            heap.push(HeapNode(next.val, *next));
        }
    }

    dummy.next
}
