use std::{cell::RefCell, cmp, rc::Rc};

use leetcode::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(1), Some(2), Some(3)]);
    let root = TreeNode::from(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let root = TreeNode::from(&[Some(2), Some(-1)]);

    println!("Result -> {}", max_path_sum(root));
}

fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut result = i32::MIN;

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, storage: &mut i32) -> i32 {
        if let Some(n) = node {
            let n_ref = n.borrow();

            let left = dfs(n_ref.left.clone(), storage);
            let right = dfs(n_ref.right.clone(), storage);

            *storage = cmp::max(*storage, left.max(0) + right.max(0) + n_ref.val);
            left.max(right).max(0) + n_ref.val
        } else {
            0
        }
    }

    dfs(root, &mut result);
    result
}
