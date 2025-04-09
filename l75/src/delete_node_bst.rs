use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let (root, key) = (
        TreeNode::from(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]),
        3,
    );

    println!("Result -> {:?}", delete_node(root, key));
}

/// - Patterns: DFS
/// - Time complexiy: $$O(n)$$
/// - Space complexiy: $$O(n)$$
fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut list = Vec::new();

    fn to_list(node: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>, key: &i32) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let node_ref = node.borrow();

        to_list(node_ref.left.clone(), list, key);

        if node_ref.val != *key {
            list.push(node_ref.val);
        }

        to_list(node_ref.right.clone(), list, key);
    }

    fn create_bst(slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if slice.is_empty() {
            return None;
        }

        let mid = slice.len() / 2;

        let mut node = TreeNode::new(slice[mid]);
        node.left = create_bst(&slice[0..mid]);
        node.right = create_bst(&slice[mid + 1..]);

        Some(Rc::new(RefCell::new(node)))
    }

    to_list(root, &mut list, &key);
    create_bst(&list)
}
