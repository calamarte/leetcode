#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let (root, val) = (
        TreeNode::from(&[Some(4), Some(2), Some(7), Some(1), Some(3)]),
        2,
    );

    println!("Result -> {:?}", search_bst_pro(root, val));
}

/// - Pattern: DFS
/// - Time complexity: $$O(log \cdot n)$$
/// - Space complexity: $$O(log \cdot n)$$
fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root.clone()?;
    let mut node_mut = node.borrow_mut();

    match node_mut.val {
        v if v == val => root,
        v if v > val => search_bst(node_mut.left.take(), val),
        v if v < val => search_bst(node_mut.right.take(), val),
        _ => None,
    }
}

/// - Pattern: BST
/// - Time complexity: $$O(log \cdot n)$$
/// - Space complexity: $$O(1)$$
fn search_bst_pro(
    mut root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    while let Some(n) = root.clone() {
        let mut node_mut = n.borrow_mut();

        if node_mut.val == val {
            return root;
        }

        root = if node_mut.val > val {
            node_mut.left.take()
        } else {
            node_mut.right.take()
        }
    }

    None
}
