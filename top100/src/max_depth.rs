///
/// # [104. Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let root = TreeNode::from(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

    println!("Result -> {}", max_depth(root));
}

/// - Patterns: dfs-iterative
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut stack = vec![(1, root.unwrap())];

    let mut layers = 0;
    while let Some((layer, node)) = stack.pop() {
        layers = layers.max(layer);

        let node_ref = node.borrow();

        if let Some(l) = node_ref.left.clone() {
            stack.push((layer + 1, l));
        }

        if let Some(r) = node_ref.right.clone() {
            stack.push((layer + 1, r));
        }
    }

    layers
}
