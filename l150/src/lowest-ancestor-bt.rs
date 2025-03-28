use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);
    let p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    let root = TreeNode::from(&[
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);
    let p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    println!(
        "Result -> {}",
        lowest_common_ancestor(root, p, q).unwrap().borrow().val
    );
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;

    if let Some(n) = root.clone() {
        let n_ref = n.borrow();

        if n_ref.val == p_val || n_ref.val == q_val {
            return root;
        }

        let left = lowest_common_ancestor(n_ref.left.clone(), p.clone(), q.clone());
        let right = lowest_common_ancestor(n_ref.right.clone(), p, q);

        if left.as_ref().and(right.as_ref()).is_some() {
            root
        } else {
            left.or(right)
        }
    } else {
        None
    }
}
