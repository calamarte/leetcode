use leetcode::{sexy_format, ListNode};

#[allow(unused_variables)]
fn main() {
    let (head, x) = (ListNode::from(&[1, 4, 3, 2, 5, 2]), 3);
    let (head, x) = (ListNode::from(&[1, 2]), 2);

    let result = partition(head, x);
    println!("Result -> {}", sexy_format(result));
}

fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut left_head = ListNode::new(-1);
    let mut right_head = ListNode::new(-1);

    let mut left_tail = &mut left_head;
    let mut right_tail = &mut right_head;

    let mut tail = &head;
    while let Some(n) = tail.as_ref() {
        let node = Some(Box::new(ListNode::new(n.val)));

        if n.val < x {
            left_tail.next = node;
            left_tail = left_tail.next.as_mut()?;
        } else {
            right_tail.next = node;
            right_tail = right_tail.next.as_mut()?;
        }

        tail = &n.next;
    }

    left_tail.next = right_head.next;
    left_head.next
}
