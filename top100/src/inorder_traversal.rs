///
/// # [94. Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(1), None, Some(2), Some(3)]);
    let root = TreeNode::from(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        Some(8),
        None,
        None,
        Some(6),
        Some(7),
        Some(9),
    ]);

    println!("Result -> {:?}", inorder_traversal(root));
}

/// - Patterns: binary-tree
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, inorder: &mut Vec<i32>) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let node_ref = node.borrow();

        dfs(node_ref.left.clone(), inorder);
        inorder.push(node_ref.val);
        dfs(node_ref.right.clone(), inorder);
    }

    let mut result = Vec::new();
    dfs(root, &mut result);
    result
}
