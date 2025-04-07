use std::{cell::RefCell, rc::Rc};

use common_lib::TreeNode;

fn main() {
    let (root, target_sum) = (
        TreeNode::from(&[
            Some(10),
            Some(5),
            Some(-3),
            Some(3),
            Some(2),
            None,
            Some(11),
            Some(3),
            Some(-2),
            None,
            Some(1),
        ]),
        8,
    );

    println!("Result -> {}", path_sum(root, target_sum));
}

/// - Patterns: DFS, Backtracking
/// - Time complexity: $$O(n^2)$$
/// - Space complexity: $$O(n)$$
fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i64>, target: i64) -> i64 {
        if let Some(node) = root {
            let val = node.borrow().val as i64;
            vec.iter_mut().for_each(|x| *x += val);
            vec.push(val);
            let max = vec.iter().filter(|&x| x == &target).count() as i64
                + dfs(node.borrow().left.as_ref(), vec, target)
                + dfs(node.borrow().right.as_ref(), vec, target);
            vec.pop();
            vec.iter_mut().for_each(|x| *x -= val);
            max
        } else {
            0
        }
    }

    dfs(root.as_ref(), &mut Vec::new(), target_sum as i64) as i32
}
