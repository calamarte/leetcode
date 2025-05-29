///
/// # [105. Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal)
///
use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let (preorder, inorder) = (vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);

    println!("Result -> {:?}", build_tree(preorder, inorder));
}

/// - Patterns: dfs
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(preo: &[i32], ino: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preo.is_empty() || ino.is_empty() {
            return None;
        }

        let mut root = TreeNode::new(preo[0]);
        let mid = ino.iter().position(|&v| v == preo[0]).unwrap();

        root.left = dfs(&preo[1..=mid], &ino[0..mid]);
        root.right = dfs(&preo[mid + 1..], &ino[mid + 1..]);

        Some(Rc::new(RefCell::new(root)))
    }

    dfs(&preorder, &inorder)
}
