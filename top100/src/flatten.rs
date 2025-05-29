///
/// # [114. Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::{right_format, TreeNode};

fn main() {
    let mut root = TreeNode::from(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);

    flatten(&mut root);
    println!("Result -> {}", right_format(root));
}

/// - Patterns: dfs
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn flat(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = node?;
        let mut node_mut = node.borrow_mut();

        let left = node_mut.left.take();
        let right = node_mut.right.take();

        let tail_left = flat(left.clone());
        let tail_right = flat(right.clone());

        node_mut.right = left;

        if let Some(tail) = &tail_left {
            tail.borrow_mut().right = right;
        } else {
            node_mut.right = right;
        }

        drop(node_mut);

        tail_right.or(tail_left).or(Some(node))
    }

    flat(root.clone());
}
