#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {}

/// - Patterns: DFS
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(h)$$
fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;

    let node = root.as_ref()?;
    let node_ref = node.borrow();

    if node_ref.val == p_val || node_ref.val == q_val {
        return root.clone();
    }

    let left = lowest_common_ancestor(node_ref.left.clone(), q.clone(), p.clone());
    let right = lowest_common_ancestor(node_ref.right.clone(), q, p);

    if left.as_ref().and(right.as_ref()).is_some() {
        root.to_owned()
    } else {
        left.or(right)
    }
}
