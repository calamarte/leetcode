use std::{cell::RefCell, cmp, rc::Rc};

use leetcode::TreeNode;

fn main() {
    let root = TreeNode::from(&[Some(4), Some(2), Some(6), Some(1), Some(3)]);

    println!("Result -> {}", get_minimun_difference(root));
}

fn get_minimun_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
    let mut min = i32::MAX;

    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        min: &mut i32,
    ) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let node_ref = node.borrow();

        dfs(node_ref.left.clone(), prev, min);

        if let Some(p) = prev {
            *min = cmp::min(*min, node_ref.val - p.borrow().val);
        }

        prev.replace(node.clone());

        dfs(node_ref.right.clone(), prev, min);
    }

    dfs(root, &mut prev, &mut min);

    min
}
