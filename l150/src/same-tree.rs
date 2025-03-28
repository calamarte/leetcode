use std::{borrow::Borrow, cell::RefCell, collections::VecDeque, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let (p, q) = (
        TreeNode::from(&[Some(1), Some(2), Some(3)]),
        TreeNode::from(&[Some(1), Some(2), Some(3)]),
    );

    println!("Result -> {}", is_same_tree(p, q));
}

// DFS
fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p.as_ref(), q.as_ref()) {
        (Some(np), Some(nq)) if RefCell::borrow(np).val == RefCell::borrow(nq).val => {
            let np_ref = RefCell::borrow(np);
            let nq_ref = RefCell::borrow(nq);

            is_same_tree(np_ref.left.clone(), nq_ref.left.clone())
                && is_same_tree(np_ref.right.clone(), nq_ref.right.clone())
        }
        (None, None) => true,
        _ => false,
    }
}

fn is_same_tree_rust(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    p == q
}
