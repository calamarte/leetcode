use std::{cell::RefCell, rc::Rc};

use leetcode::TreeNode;

fn main() {
    let root = TreeNode::from(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);

    println!("Result -> {}", count_nodes(root));
}

fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(n) = root {
        let left = n.borrow().left.clone();
        let right = n.borrow().right.clone();

        1 + count_nodes(left) + count_nodes(right)
    } else {
        0
    }
}
