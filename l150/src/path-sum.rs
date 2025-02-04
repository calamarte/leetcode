use std::{cell::RefCell, rc::Rc};

use leetcode::TreeNode;

#[allow(unused_variables)]
fn main() {
    let (root, target) = (
        TreeNode::from(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]),
        22,
    );

    let (root, target) = (TreeNode::from(&[Some(1), Some(2), Some(3)]), 5);
    let (root, target) = (TreeNode::from(&[Some(1), Some(2)]), 1);
    let (root, target) = (TreeNode::from(&[Some(-2), None, Some(-3)]), -5);

    println!("Result -> {}", has_path_sum(root, target));
}

fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }

    fn dfs(root: Rc<RefCell<TreeNode>>, target_sum: i32, mut acc: i32) -> bool {
        let root_ref = root.borrow();

        acc += root_ref.val;

        match (root_ref.left.clone(), root_ref.right.clone()) {
            (Some(ln), Some(rn)) => {
                let left = dfs(ln, target_sum, acc);
                let right = dfs(rn, target_sum, acc);

                left || right
            }
            (Some(ln), None) => dfs(ln, target_sum, acc),
            (None, Some(rn)) => dfs(rn, target_sum, acc),
            (None, None) => acc == target_sum,
        }
    }

    dfs(root.unwrap(), target_sum, 0)
}
