///
/// # [98. Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/)
///
use common_lib::TreeNode;
use std::{cell::RefCell, rc::Rc};

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(2), Some(1), Some(3)]);
    let root = TreeNode::from(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);

    println!("Result -> {}", is_valid_bst(root));
}

type Node = Rc<RefCell<TreeNode>>;

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack: Vec<(Node, i64, i64)> = vec![(root.expect("Not empty!"), i64::MIN, i64::MAX)];

    while let Some((node, left, right)) = stack.pop() {
        let mut node_mut = node.borrow_mut();

        let val = node_mut.val as i64;
        if left >= val || val >= right {
            return false;
        }

        if let Some(l) = node_mut.left.take() {
            stack.push((l, left, val));
        }

        if let Some(r) = node_mut.right.take() {
            stack.push((r, val, right));
        }
    }

    true
}
