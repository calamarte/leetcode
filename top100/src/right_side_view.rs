///
/// # [199. Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view/)
///
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(4)]);
    let root = TreeNode::from(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        None,
        Some(5),
    ]);

    println!("Result -> {:?}", right_side_view(root));
}

/// - Patterns: bfs
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(max(w, h)$$
fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let root = match root {
        Some(r) => r,
        None => return Vec::new(),
    };

    let mut deque = VecDeque::new();
    let mut result = Vec::new();

    deque.push_front(root);

    while !deque.is_empty() {
        let next_value = deque.back().unwrap().borrow().val;

        result.push(next_value);

        for _ in 0..deque.len() {
            let next = deque.pop_back().unwrap();
            let next_ref = next.borrow();

            if let Some(r) = next_ref.right.clone() {
                deque.push_front(r);
            }

            if let Some(l) = next_ref.left.clone() {
                deque.push_front(l);
            }
        }
    }

    result
}
