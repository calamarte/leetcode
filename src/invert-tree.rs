use std::{cell::RefCell, rc::Rc};

use leetcode::TreeNode;

fn main() {
    let root = TreeNode::from(&[
        Some(4),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(6),
        Some(9),
    ]);

    println!("{:?}", invert_tree(root));
}

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root.clone()?;
    let mut node_mut = node.borrow_mut();

    let left = node_mut.left.clone();
    node_mut.left = node_mut.right.clone();
    node_mut.right = left;

    invert_tree(node_mut.left.clone());
    invert_tree(node_mut.right.clone());

    root
}
