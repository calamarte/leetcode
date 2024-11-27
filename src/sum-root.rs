use std::{cell::RefCell, rc::Rc};

use leetcode::TreeNode;

fn main() {
    let root = TreeNode::from(&[Some(1), Some(2), Some(3)]);

    println!("Result -> {}", sum_number(root));
}

fn sum_number(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut result = 0;
    fn dfs(node: Rc<RefCell<TreeNode>>, mut num: i32, storage: &mut i32) {
        let node_ref = node.borrow();
        num = num * 10 + node_ref.val;

        if node_ref.left.is_none() && node_ref.right.is_none() {
            *storage += num;
            return;
        }

        if let Some(ln) = node_ref.left.clone() {
            dfs(ln, num, storage);
        }

        if let Some(rn) = node_ref.right.clone() {
            dfs(rn, num, storage);
        }
    }

    dfs(root.unwrap(), 0, &mut result);

    result
}
