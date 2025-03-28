use std::{cell::RefCell, rc::Rc};

use common_lib::{right_format, TreeNode};

#[allow(unused_variables, unused_mut)]
fn main() {
    let mut root = TreeNode::from(&[Some(1), Some(2), Some(5)]);
    let mut root = TreeNode::from(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);

    flatten(&mut root);

    println!("Result -> {}", right_format(root.clone()));
}

trait AppendNode {
    fn append(&mut self, to_append: Option<Rc<RefCell<TreeNode>>>);
}

impl AppendNode for TreeNode {
    fn append(&mut self, to_append: Option<Rc<RefCell<TreeNode>>>) {
        if to_append.is_none() {
            return;
        }

        if self.right.is_none() {
            self.right = to_append;
            return;
        }

        let mut tail = self.right.clone();
        let mut last = tail.clone().unwrap();
        while let Some(node) = tail {
            last = node.clone();
            tail = node.borrow().right.clone();
        }

        last.borrow_mut().right = to_append;
    }
}

fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        return;
    }

    let root = root.clone().unwrap();
    let mut root_mut = root.borrow_mut();

    let mut left = root_mut.left.take();
    let mut right = root_mut.right.take();

    flatten(&mut left);
    flatten(&mut right);

    if let Some(ln) = left.clone() {
        ln.borrow_mut().append(right.clone());
        root_mut.right = left;
    } else {
        root_mut.right = right;
    }
}
