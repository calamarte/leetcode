use std::iter::successors;

use common_lib::{sexy_format, ListNode};

fn main() {
    let head = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(5))),
                })),
            })),
        })),
    };

    let (left, right) = (2, 4);

    let result = reverse_between(Some(Box::new(head)), left, right);

    println!("Result -> {}", sexy_format(result));
}

fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut list: Vec<_> = successors(head.as_ref(), |n| n.next.as_ref())
        .map(|n| n.val)
        .collect();

    list[left as usize - 1..right as usize].reverse();

    let mut dummy = ListNode::new(-1);
    list.into_iter().fold(&mut dummy, |acc, v| {
        acc.next = Some(Box::new(ListNode::new(v)));
        acc.next.as_mut().unwrap()
    });

    dummy.next
}
