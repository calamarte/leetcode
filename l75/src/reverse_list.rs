#![allow(dead_code)]

use common_lib::{sexy_format, ListNode};

fn main() {
    let head = ListNode::from(&[1, 2, 3, 4, 5]);

    println!("Result -> {}", sexy_format(reverse_list_pro(head)));
}

/// - Patterns: linkedlist
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack = Vec::new();

    while let Some(n) = head {
        stack.push(n.val);
        head = n.next;
    }

    let mut dummy = ListNode::new(-1);
    let mut tail = &mut dummy;

    for v in stack.into_iter().rev() {
        let n = Some(Box::new(ListNode::new(v)));

        tail.next = n;
        tail = tail.next.as_mut().unwrap();
    }

    dummy.next
}

/// - Patterns: linkedlist
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn reverse_list_pro(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next;
        node.next = prev;
        prev = Some(node);
    }

    prev
}
