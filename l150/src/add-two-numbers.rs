use common_lib::{sexy_format, ListNode};

fn main() {
    let l1 = ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    };

    let l2 = ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };

    let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));

    println!("Result -> {}", sexy_format(result));
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (&l1, &l2) {
        (None, None) => return None,
        (Some(_), None) => return l1,
        (None, Some(_)) => return l2,
        _ => (),
    }

    let (mut a, mut b) = (l1.as_ref(), l2.as_ref());

    let mut result: Option<Box<ListNode>> = None;
    let mut tail = &mut result;
    let mut acc = 0;
    while a.is_some() || b.is_some() {
        let next_result = match (a, b) {
            (Some(v1), Some(v2)) => {
                let total = v1.val + v2.val + acc;
                acc = total / 10;

                a = v1.next.as_ref();
                b = v2.next.as_ref();

                total % 10
            }
            (Some(v1), None) => {
                let total = v1.val + acc;
                acc = total / 10;

                a = v1.next.as_ref();

                total % 10
            }
            (None, Some(v2)) => {
                let total = v2.val + acc;
                acc = total / 10;

                b = v2.next.as_ref();

                total % 10
            }
            _ => panic!("Never here!"),
        };

        let new_node = Box::new(ListNode::new(next_result));

        if let Some(n) = tail {
            n.next = Some(new_node);
            tail = &mut n.next;
        } else {
            tail.replace(new_node);
        }
    }

    if acc > 0 {
        let new_node = Box::new(ListNode::new(acc));

        if let Some(n) = tail {
            n.next = Some(new_node);
        } else {
            tail.replace(new_node);
        }
    }

    result
}
