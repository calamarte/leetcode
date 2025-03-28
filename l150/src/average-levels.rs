use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let root = TreeNode::from(&[Some(3), Some(9), Some(20), Some(15), Some(7)]);

    println!("Result -> {:?}", average_of_levels(root));
}

fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    if root.is_none() {
        return Vec::new();
    }

    let mut averages: Vec<f64> = Vec::new();
    let mut deq = VecDeque::from([root.unwrap()]);

    while !deq.is_empty() {
        let len = deq.len();

        let mut sum = 0.0;

        for _ in 0..len {
            let node = deq.pop_front().unwrap();
            let node_ref = node.borrow();

            sum += node_ref.val as f64;

            if let Some(l) = node_ref.left.clone() {
                deq.push_back(l);
            }

            if let Some(r) = node_ref.right.clone() {
                deq.push_back(r);
            }
        }

        averages.push(sum / len as f64);
    }

    averages
}
