use std::iter::successors;

use leetcode::{sexy_format, ListNode};

fn main() {
    let (head, n) = (ListNode::from(&[1, 2, 3, 4, 5]), 2);

    let result = remove_nth_from_end(head, n);

    println!("Result -> {}", sexy_format(result));
}

fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut target = head.clone();
    let mut node = &mut head;

    for _ in 0..n {
        target = target.map(|nod| nod.next)?;
    }

    while let Some(nod) = target {
        target = nod.next;
        node = &mut node.as_mut()?.next;
    }

    *node = node.as_mut().and_then(|nod| nod.next.take());

    head
}
