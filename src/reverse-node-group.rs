use std::iter::successors;

use leetcode::{sexy_format, ListNode};

fn main() {
    let (head, k) = (ListNode::from(&[1, 2, 3, 4, 5]), 2);

    let result = reverse_k_group(head, k);
    println!("Result -> {}", sexy_format(result));
}

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut vec = successors(head.as_ref(), |n| n.next.as_ref())
        .map(|n| n.val)
        .collect::<Vec<_>>();

    vec.chunks_mut(k as usize)
        .filter(|c| c.len() % k as usize == 0)
        .for_each(|c| c.reverse());

    let mut dummy = ListNode::new(-1);
    vec.into_iter().fold(&mut dummy, |tail, v| {
        tail.next = Some(Box::new(ListNode::new(v)));
        tail.next.as_mut().unwrap()
    });

    dummy.next
}
