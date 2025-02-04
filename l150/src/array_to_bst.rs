use std::{cell::RefCell, rc::Rc};

use leetcode::TreeNode;

fn main() {
    let nums = vec![-10, -3, 0, 5, 9];

    let result = sorted_array_to_bst(nums);

    println!("{:?}", result);
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(n)$$
fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;
        let mut mid_node = TreeNode::new(nums[mid]);

        mid_node.left = dfs(&nums[..mid]);
        mid_node.right = dfs(&nums[mid + 1..]);

        Some(Rc::new(RefCell::new(mid_node)))
    }

    dfs(&nums)
}
