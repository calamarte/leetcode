use common_lib::{sexy_format, ListNode};

#[allow(unused_variables)]
fn main() {
    let head = ListNode::from(&[1, 2, 3, 4, 5]);
    let head = ListNode::from(&[2, 1, 3, 5, 6, 4, 7]);

    println!("Result -> {}", sexy_format(odd_even_list(head)));
}

/// - Pattern: linkedlist
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_odd = ListNode::new(-1);
    let mut dummy_even = ListNode::new(-1);

    let mut odd = &mut dummy_odd;
    let mut even = &mut dummy_even;

    let mut is_even = false;
    while let Some(node) = head {
        let n_node = Some(Box::new(ListNode::new(node.val)));

        if is_even {
            even.next = n_node;
            even = even.next.as_mut().unwrap();
        } else {
            odd.next = n_node;
            odd = odd.next.as_mut().unwrap();
        }

        is_even = !is_even;
        head = node.next;
    }

    odd.next = dummy_even.next;
    dummy_odd.next
}
