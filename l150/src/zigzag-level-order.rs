use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let root = TreeNode::from(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

    println!("Result -> {:?}", zigzag_level_order(root));
}

fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return Vec::new();
    }

    let mut deq = VecDeque::from([root.unwrap()]);
    let mut result = Vec::new();
    let mut deph = 1;

    while !deq.is_empty() {
        let len = deq.len();
        let mut level = Vec::new();

        for _ in 0..len {
            let node = deq.pop_front().unwrap();
            let node_ref = node.borrow();

            level.push(node_ref.val);

            if let Some(l) = node_ref.left.clone() {
                deq.push_back(l);
            }

            if let Some(r) = node_ref.right.clone() {
                deq.push_back(r);
            }
        }

        if deph % 2 == 0 {
            level.reverse();
        }

        result.push(level);
        deph += 1;
    }

    result
}
