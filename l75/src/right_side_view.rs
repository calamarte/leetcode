use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(1), Some(2), Some(3), None, Some(5), Some(4)]);
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

/// - Patterns: BFS
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut layers = VecDeque::new();
    let mut result = Vec::new();

    layers.push_back((1, root));

    while let Some((l, n)) = layers.pop_back() {
        if n.is_none() {
            continue;
        }

        let node = n.unwrap();
        let mut node_mut = node.borrow_mut();

        if l as usize > result.len() {
            result.push(node_mut.val);
        }

        layers.push_front((l + 1, node_mut.right.take()));
        layers.push_front((l + 1, node_mut.left.take()));
    }

    result
}
