use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use common_lib::TreeNode;

fn main() {
    let root = TreeNode::from(&[Some(1), Some(7), Some(0), Some(7), Some(-8), None, None]);

    println!("Result -> {}", max_level_sum(root));
}

/// - Patterns: BFS
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut layers = VecDeque::new();
    let mut layers_sum: HashMap<usize, i32> = HashMap::new();

    layers.push_front((1, root));

    while let Some((l, n)) = layers.pop_back() {
        if n.is_none() {
            continue;
        }

        let node = n.unwrap();
        let mut node_mut = node.borrow_mut();

        *layers_sum.entry(l).or_default() += node_mut.val;

        layers.push_front((l + 1, node_mut.left.take()));
        layers.push_front((l + 1, node_mut.right.take()));
    }

    layers_sum
        .into_iter()
        .max_by(|a, b| match a.1.cmp(&b.1) {
            Ordering::Equal => b.0.cmp(&a.0),
            v => v,
        })
        .expect("Never empty!")
        .0 as i32
}

