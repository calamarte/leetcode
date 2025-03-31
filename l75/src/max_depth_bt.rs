#![allow(dead_code)]

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    // let root = TreeNode::from(&[Some(1), None, Some(2)]);

    println!("Result -> {}", max_depth(root));
}

/// - Patterns: DFS
/// - Time compplexity: $$O(n)$$
/// - Space compplexity: $$O(n)$$
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut layer: i32) -> i32 {
        if node.is_none() {
            return layer;
        }

        layer += 1;

        let node = node.unwrap();
        let node_ref = node.borrow();

        dfs(node_ref.left.clone(), layer).max(dfs(node_ref.right.clone(), layer))
    }

    dfs(root, 0)
}

/// - Patterns: BFS
/// - Time compplexity: $$O(n)$$
/// - Space compplexity: $$O(n)$$
fn max_depth_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut layer = VecDeque::new();
    layer.push_front((1, root.unwrap()));

    let mut max_depth = 0;
    while let Some((d, n)) = layer.pop_back() {
        max_depth = max_depth.max(d);

        let node_ref = n.borrow();

        if let Some(l) = node_ref.left.clone() {
            layer.push_front((d + 1, l));
        }

        if let Some(r) = node_ref.right.clone() {
            layer.push_front((d + 1, r));
        }
    }

    max_depth
}
