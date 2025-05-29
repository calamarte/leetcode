///
/// # [230. Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let (root, k) = (
        TreeNode::from(&[Some(3), Some(1), Some(4), None, Some(2)]),
        1,
    );

    let (root, k) = (
        TreeNode::from(&[
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ]),
        3,
    );

    println!("Result -> {}", kth_smallest(root, k));
}

/// - Patterns: dfs
/// - Time complexity: $$O(k)$$
/// - Space complexity: $$O(h)$$
fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, counter: &mut i32, found: &mut Option<i32>) {
        if node.is_none() || found.is_some() {
            return;
        }

        let node = node.unwrap();
        let node_ref = node.borrow();

        dfs(node_ref.left.clone(), counter, found);

        *counter -= 1;

        if *counter == 0 {
            *found = Some(node_ref.val);
            return;
        }

        dfs(node_ref.right.clone(), counter, found);
    }

    let mut result = None;
    dfs(root, &mut k, &mut result);

    result.expect("Some value!")
}
