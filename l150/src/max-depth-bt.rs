use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use common_lib::TreeNode;

#[allow(unused_variables)]
fn main() {
    let root = TreeNode::from(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    // let root = TreeNode::from(&[Some(1), None, Some(2)]);

    println!("Result -> {}", max_depth_bfs(root));
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    max_depth_rec(root, 1) as i32
}

fn max_depth_rec(root: Option<Rc<RefCell<TreeNode>>>, deep: usize) -> usize {
    if let Some(n) = root {
        let n_ref = n.borrow();
        let left = n_ref.left.clone();
        let right = n_ref.right.clone();
        match (left.as_ref(), right.as_ref()) {
            (Some(_), Some(_)) => max_depth_rec(left, deep + 1).max(max_depth_rec(right, deep + 1)),
            (Some(_), None) => max_depth_rec(left, deep + 1),
            (None, Some(_)) => max_depth_rec(right, deep + 1),
            _ => deep,
        }
    } else {
        deep
    }
}

fn max_depth_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut level: usize = 0;
    let mut deq = VecDeque::new();
    deq.push_front(root.unwrap());

    while !deq.is_empty() {
        for _ in 0..deq.len() {
            let node = deq.pop_front().unwrap();
            let node_ref = node.borrow();

            if let Some(l) = node_ref.left.clone() {
                deq.push_back(l);
            }

            if let Some(r) = node_ref.right.clone() {
                deq.push_back(r);
            }
        }

        level += 1;
    }

    level as i32
}

fn max_depth_int_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut result = 0;
    let mut stack = vec![(root, 1)];
    while let Some((node, depth)) = stack.pop() {
        if let Some(n) = node {
            let n = n.borrow();
            result = result.max(depth);
            stack.push((n.left.clone(), depth + 1));
            stack.push((n.right.clone(), depth + 1));
        }
    }

    result
}
