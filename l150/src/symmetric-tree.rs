use leetcode::TreeNode;
use std::{cell::RefCell, rc::Rc};

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(4),
        Some(4),
        Some(3),
    ]);

    let root = TreeNode::from(&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]);

    println!("Result -> {}", is_symmetric(root));
}

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }

    let node = root.unwrap();
    let node_ref = RefCell::borrow(&node);

    dfs(node_ref.left.clone(), node_ref.right.clone())
}

fn dfs(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (left, right) {
        (Some(ln), Some(rn)) if RefCell::borrow(&ln).val == RefCell::borrow(&rn).val => {
            let ln_ref = RefCell::borrow(&ln);
            let rn_ref = RefCell::borrow(&rn);

            dfs(ln_ref.left.clone(), rn_ref.right.clone())
                && dfs(ln_ref.right.clone(), rn_ref.left.clone())
        }
        (None, None) => true,
        _ => false,
    }
}
