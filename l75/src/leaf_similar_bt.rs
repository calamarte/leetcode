use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {
    #[rustfmt::skip]
    let (root1, root2) = (
        TreeNode::from(&[Some(3), Some(5), Some(1), Some(6), Some(2), Some(9), Some(8), None, None, Some(7), Some(4)]),
        TreeNode::from(&[Some(3), Some(5), Some(1), Some(6), Some(7), Some(4), Some(2), None, None, None, None, None, None, Some(9), Some(8)])
    );

    println!("Result -> {}", leaf_similar(root1, root2));
}

/// - Patterns: DFS
/// - Time complexity: $$O(n + m)$$
/// - Space complexity: $$O(n + m)$$
fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    fn dfs(node: Rc<RefCell<TreeNode>>, leaf: &mut Vec<i32>) {
        let node_ref = node.borrow();

        if node_ref.left.is_none() && node_ref.right.is_none() {
            leaf.push(node_ref.val);
            return;
        }

        if let Some(l) = node_ref.left.clone() {
            dfs(l, leaf);
        }

        if let Some(r) = node_ref.right.clone() {
            dfs(r, leaf);
        }
    }

    let mut leaf1 = Vec::new();
    if let Some(r1) = root1 {
        dfs(r1, &mut leaf1);
    }

    let mut leaf2 = Vec::new();
    if let Some(r2) = root2 {
        dfs(r2, &mut leaf2);
    }

    leaf1 == leaf2
}
