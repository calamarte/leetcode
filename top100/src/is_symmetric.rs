///
///  # [101. Symmetric Tree](https://leetcode.com/problems/symmetric-tree/)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let root = TreeNode::from(&[
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(4),
        Some(4),
        Some(3),
    ]);

    println!("Result -> {}", is_symmetric(root));
}

/// - Patterns: dfs
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }

    fn inverse(node: Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let mut node_mut = node.borrow_mut();
        (node_mut.left, node_mut.right) = (node_mut.right.take(), node_mut.left.take());

        inverse(node_mut.left.clone());
        inverse(node_mut.right.clone());
    }

    let node = root.unwrap();
    let node_ref = node.borrow();

    inverse(node_ref.right.clone());

    node_ref.left == node_ref.right
}
