use common_lib::{sexy_format, ListNode};
use std::iter::successors;

fn main() {
    let (head, k) = (ListNode::from(&[1, 2, 3, 4, 5]), 2);

    let result = rotate_right(head, k);
    println!("Result -> {}", sexy_format(result));
}

fn rotate_right(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
    head.as_ref()?;

    let vec: Vec<_> = successors(head.as_ref(), |n| n.next.as_ref())
        .map(|n| n.val)
        .collect();

    let len = vec.len();

    if k != 0 {
        k %= len as i32
    }

    let skip = len - k as usize;
    let mut dummy = ListNode::new(-1);
    vec.into_iter()
        .cycle()
        .skip(skip)
        .take(len)
        .fold(&mut dummy, |tail, v| {
            tail.next = Some(Box::new(ListNode::new(v)));
            tail.next.as_mut().unwrap()
        });

    dummy.next
}
