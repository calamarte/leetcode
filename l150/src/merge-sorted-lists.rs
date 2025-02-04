use core::panic;

use leetcode::{sexy_format, ListNode};

fn main() {
    let l1 = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };

    let l2 = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };

    let result = merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));

    println!("Result -> {}", sexy_format(result));
}

fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (&list1, &list2) {
        (None, None) => return None,
        (Some(_), None) => return list1,
        (None, Some(_)) => return list2,
        _ => (),
    }

    let (mut a, mut b) = (list1.as_ref(), list2.as_ref());

    let mut result: Option<Box<ListNode>> = None;
    let mut tail = &mut result;
    while a.is_some() || b.is_some() {
        let new_value = match (a, b) {
            (Some(n1), Some(n2)) => {
                if n1.val < n2.val {
                    a = n1.next.as_ref();
                    n1.val
                } else {
                    b = n2.next.as_ref();
                    n2.val
                }
            }
            (Some(n1), None) => {
                if let Some(t) = tail {
                    t.next = Some(n1.to_owned());
                } else {
                    *tail = Some(n1.to_owned());
                }

                break;
            }
            (None, Some(n2)) => {
                if let Some(t) = tail {
                    t.next = Some(n2.to_owned());
                } else {
                    *tail = Some(n2.to_owned());
                }

                break;
            }
            _ => panic!("Never here!"),
        };

        let new_node = Box::new(ListNode::new(new_value));

        if let Some(n) = tail {
            n.next = Some(new_node);
            tail = &mut n.next;
        } else {
            tail.replace(new_node);
        }
    }

    result
}
