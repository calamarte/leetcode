///
/// # [108. Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let nums = vec![-10, -3, 0, 5, 9];

    println!("Result -> {:?}", sorted_array_to_bst(nums));
}

/// - Patterns: dfs
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }

        let mid = values.len() / 2;
        let mut node = TreeNode::new(values[mid]);

        node.left = dfs(&values[0..mid]);
        node.right = dfs(&values[mid + 1..]);

        Some(Rc::new(RefCell::new(node)))
    }

    dfs(&nums)
}
