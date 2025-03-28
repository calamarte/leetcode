use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let (preorder, inorder) = (vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);

    println!("{:?}", build_tree(preorder, inorder));
}

fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_slice(&preorder, &inorder)
}

fn build_tree_slice(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }

    let mut root = TreeNode::new(preorder[0]);
    let mid = inorder.iter().position(|&x| x == preorder[0])?;

    root.left = build_tree_slice(&preorder[1..=mid], &inorder[..=mid]);
    root.right = build_tree_slice(&preorder[mid + 1..], &inorder[mid + 1..]);

    Some(Rc::new(RefCell::new(root)))
}
