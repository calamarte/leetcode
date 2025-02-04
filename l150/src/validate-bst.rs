use std::{cell::RefCell, rc::Rc};

use leetcode::TreeNode;

fn main() {
    let root = TreeNode::from(&[Some(2), Some(1), Some(3)]);

    println!("Result -> {}", is_valid_bst(root));
}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut vec = Vec::new();

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, storage: &mut Vec<i32>) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let node_ref = node.borrow();

        dfs(node_ref.left.clone(), storage);
        storage.push(node_ref.val);
        dfs(node_ref.right.clone(), storage);
    }

    dfs(root, &mut vec);

    vec.is_sorted()

    // Leetcode manual solution don't allow is_sorted
    // for window in vec.windows(2) {
    //     if window[0] >= window[1] {
    //         return false;
    //     }
    // }
    //
    // true
}
