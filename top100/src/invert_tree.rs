///
/// # [226. Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

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

    println!("Result -> {:?}", invert_tree(root));
}

/// - Patterns: dfs-iterative
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(h)$$
fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return root;
    }

    let root = root.unwrap();
    let mut stack = vec![root.clone()];

    while let Some(node) = stack.pop() {
        let mut node_mut = node.borrow_mut();
        (node_mut.left, node_mut.right) = (node_mut.right.clone(), node_mut.left.clone());

        if let Some(l) = node_mut.left.clone() {
            stack.push(l);
        }

        if let Some(r) = node_mut.right.clone() {
            stack.push(r);
        }
    }

    Some(root)
}
