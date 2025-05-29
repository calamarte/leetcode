///
/// #  [102. Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)
///
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let root = TreeNode::from(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

    println!("Result -> {:?}", level_order(root));
}

/// - Patterns: bfs
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return Vec::new();
    }

    let root = root.unwrap();
    let mut que = VecDeque::new();
    let mut result: Vec<Vec<i32>> = Vec::new();

    que.push_front((0, root.clone()));

    while let Some((deep, node)) = que.pop_back() {
        let node_ref = node.borrow();

        if let Some(v) = result.get_mut(deep) {
            v.push(node_ref.val);
        } else {
            result.push(vec![node_ref.val]);
        }

        if let Some(l) = node_ref.left.clone() {
            que.push_front((deep + 1, l));
        }

        if let Some(r) = node_ref.right.clone() {
            que.push_front((deep + 1, r));
        }
    }

    result
}
