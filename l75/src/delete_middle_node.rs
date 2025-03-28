use common_lib::{sexy_format, ListNode};

#[allow(unused_variables)]
fn main() {
    let head = ListNode::from(&[1, 3, 4, 7, 1, 2, 6]);
    let head = ListNode::from(&[1, 2, 3, 4]);

    println!("Result -> {}", sexy_format(delete_middle(head)));
}

/// - Pattern: fast-slow
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return None;
    }

    let mut dummy = Box::new(ListNode::new(-1));
    dummy.next = head.clone(); // NOTE: Is posible avoid clone?

    let mut slow = &mut dummy;
    let mut fast = head.unwrap();

    while let Some(f) = fast.next {
        slow = slow.next.as_mut().unwrap();

        if f.next.is_some() {
            fast = f.next.unwrap();
        } else {
            break;
        }
    }

    slow.next = slow.next.as_mut().unwrap().next.take();

    dummy.next
}
