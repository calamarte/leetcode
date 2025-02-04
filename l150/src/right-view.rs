use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use leetcode::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(4)]);
    let root = TreeNode::from(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        Some(5),
    ]);
    let root = TreeNode::from(&[Some(1), None, Some(3)]);
    let root = TreeNode::from(&[]);

    println!("Result -> {:?}", right_side_view(root));
}

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return Vec::new();
    }

    let mut side_view = Vec::new();
    let mut deq = VecDeque::from([root.unwrap()]);

    while !deq.is_empty() {
        let len = deq.len();

        let last = deq.back().unwrap().borrow().val;
        side_view.push(last);

        for _ in 0..len {
            let node = deq.pop_front().unwrap();
            let node_ref = node.borrow();

            if let Some(l) = node_ref.left.clone() {
                deq.push_back(l);
            }

            if let Some(r) = node_ref.right.clone() {
                deq.push_back(r);
            }
        }
    }

    side_view
}
