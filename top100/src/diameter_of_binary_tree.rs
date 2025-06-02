///
/// # [543. Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree/)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
    let root = TreeNode::from(&[Some(1), Some(2)]);

    println!("Result -> {}", diameter_of_binary_tree(root));
}

/// - Patterns: dfs
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(h)$$
fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max_len: &mut i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node = node.unwrap();
        let node_ref = node.borrow();

        let left = dfs(node_ref.left.clone(), max_len);
        let right = dfs(node_ref.right.clone(), max_len);

        let len = left + right + 1;
        *max_len = len.max(*max_len);

        left.max(right) + 1
    }

    let mut max_len = 0;
    dfs(root, &mut max_len);
    max_len - 1
}
