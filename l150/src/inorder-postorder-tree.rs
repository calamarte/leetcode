use std::{cell::RefCell, collections::HashMap, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let (inorder, postorder) = (vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);

    println!("Result -> {:?}", build_tree(inorder, postorder));
}

fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build_tree_slice(
        inorder: &[i32],
        postorder: &mut Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }

        let mut root = TreeNode::new(postorder.pop()?);
        let mid = inorder.iter().position(|&v| v == root.val)?;

        root.right = build_tree_slice(&inorder[mid + 1..], postorder);
        root.left = build_tree_slice(&inorder[..mid], postorder);

        Some(Rc::new(RefCell::new(root)))
    }

    build_tree_slice(&inorder, &mut postorder)
}
