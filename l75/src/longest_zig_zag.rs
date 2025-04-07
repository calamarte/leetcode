use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[
        Some(1),
        None,
        Some(1),
        Some(1),
        Some(1),
        None,
        None,
        Some(1),
        Some(1),
        None,
        Some(1),
        None,
        None,
        None,
        Some(1),
    ]);
    let root = TreeNode::from(&[
        Some(1),
        Some(1),
        Some(1),
        None,
        Some(1),
        None,
        None,
        Some(1),
        Some(1),
        None,
        Some(1),
    ]);

    println!("Result -> {}", longest_zig_zag(root));
}

// - Patterns: DFS
// - Time complexity: $$O(n)$$
// - Space complexity: $$O(n)$$
fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn drill(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool, mut len: u32, max_len: &mut u32) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let node_ref = node.borrow();

        len += 1;
        *max_len = len.max(*max_len);

        if is_left {
            drill(node_ref.left.clone(), !is_left, len, max_len);
            drill(node_ref.right.clone(), is_left, 1, max_len);
        } else {
            drill(node_ref.right.clone(), !is_left, len, max_len);
            drill(node_ref.left.clone(), is_left, 1, max_len);
        }
    }

    let mut max_nodes = 0;
    drill(root, true, 0, &mut max_nodes);

    max_nodes as i32 - 1
}
