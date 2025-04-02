use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)]);
    let root = TreeNode::from(&[Some(3), Some(3), None, Some(4), Some(2)]);
    let root = TreeNode::from(&[Some(9), None, Some(3), Some(6)]);

    println!("Result -> {}", good_nodes(root));
}

/// - Patterns: DFS
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut max: i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node = node.unwrap();
        let node_ref = node.borrow();

        max = max.max(node_ref.val);

        let count = if node_ref.val == max { 1 } else { 0 };

        count + dfs(node_ref.left.clone(), max) + dfs(node_ref.right.clone(), max)
    }

    dfs(root, i32::MIN)
}
