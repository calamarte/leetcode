use std::{
    collections::{BTreeSet, HashSet},
    iter::successors,
};

use common_lib::{sexy_format, ListNode};

fn main() {
    let head = ListNode::from(&[1, 2, 3, 3, 4, 4, 5]);

    let result = delete_duplicates(head);
    println!("Result -> {}", sexy_format(result));
}

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode {
        val: -1,
        next: head,
    };
    let mut tail = &mut dummy;

    while let Some(mut n) = tail.next.as_mut() {
        if n.next.is_some() && n.val == n.next.as_ref().unwrap().val {
            while n.next.is_some() && n.val == n.next.as_ref().unwrap().val {
                n = n.next.as_mut().unwrap();
            }
            tail.next = n.next.take();
        } else {
            tail = tail.next.as_mut().unwrap();
        }
    }

    dummy.next
}

#[allow(dead_code)]
fn delete_duplicates_rust(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut set: HashSet<i32> = HashSet::new();
    let mut result_set: BTreeSet<i32> = BTreeSet::new();
    successors(head.as_ref(), |n| n.next.as_ref())
        .map(|n| n.val)
        .for_each(|v| {
            if !set.insert(v) {
                result_set.remove(&v);
            } else {
                result_set.insert(v);
            }
        });

    let mut dummy = ListNode::new(-1);
    result_set.into_iter().fold(&mut dummy, |tail, v| {
        tail.next = Some(Box::new(ListNode::new(v)));
        tail.next.as_mut().unwrap()
    });

    dummy.next
}
