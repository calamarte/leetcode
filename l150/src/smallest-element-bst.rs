#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

use leetcode::TreeNode;

fn main() {
    let (root, k) = (
        TreeNode::from(&[Some(3), Some(1), Some(4), None, Some(2)]),
        1,
    );

    println!("Result -> {}", kth_smallest_stack(root, k));
}

fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut vec: Vec<i32> = Vec::new();

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, k: i32, storage: &mut Vec<i32>) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let node_ref = node.borrow();

        dfs(node_ref.left.clone(), k, storage);

        if storage.len() == k as usize {
            return;
        }

        storage.push(node_ref.val);

        dfs(node_ref.right.clone(), k, storage);
    }

    dfs(root, k, &mut vec);

    *vec.last().unwrap()
}

fn kth_smallest_stack(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut visited = 0;
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut node = root;

    while !stack.is_empty() || node.is_some() {
        while let Some(n) = node.clone() {
            stack.push(n.clone());
            node = n.borrow().left.clone();
        }

        let n = stack.pop().unwrap();
        let n_ref = n.borrow();

        visited += 1;
        if visited == k {
            return n_ref.val;
        }

        node = n_ref.right.clone();
    }

    -1
}
