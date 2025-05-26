///
/// # [124. Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(1), Some(2), Some(3)]);
    let root = TreeNode::from(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let root = TreeNode::from(&[Some(0)]);
    let root = TreeNode::from(&[Some(1), Some(2)]);
    let root = TreeNode::from(&[Some(2), Some(0)]);
    let root = TreeNode::from(&[Some(0)]);
    let root = TreeNode::from(&[Some(2), Some(-1)]);

    println!("Result (Not work!) -> {}", max_path_sum(root.clone()));
    println!("Result (leetcode) -> {}", max_path_sum_pro(root.clone()));
}

/// - Patterns: binary-tree
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
// FIXME: My failed attempt :(
fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Rc<RefCell<TreeNode>>, max_sum: &mut i32) -> i32 {
        let node_ref = node.borrow();

        let left = node_ref
            .left
            .as_ref()
            .map(|l| dfs(l.clone(), max_sum))
            .unwrap_or_default();

        let right = node_ref
            .right
            .as_ref()
            .map(|r| dfs(r.clone(), max_sum))
            .unwrap_or_default();

        *max_sum = (*max_sum).max(node_ref.val + left + right);

        node_ref.val + left.max(right)
    }

    let root = root.unwrap();
    let mut max_sum = i32::MIN;
    dfs(root.clone(), &mut max_sum);

    max_sum
}

/// - Patterns: binary-tree
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn max_path_sum_pro(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = i32::MIN;
    fn rec(max: &mut i32, node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = node {
            let mut node = node.borrow_mut();
            let max_from_left = rec(max, node.left.take()).max(0);
            let max_from_right = rec(max, node.right.take()).max(0);
            *max = (*max).max(max_from_left + node.val + max_from_right);
            node.val + max_from_left.max(max_from_right)
        } else {
            0
        }
    }
    max = rec(&mut max, root).max(max);
    max
}
