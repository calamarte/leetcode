use std::{cell::RefCell, iter::Peekable, rc::Rc, vec};

use leetcode::TreeNode;

fn main() {
    let root = TreeNode::from(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20)]);

    let mut iter = BSTIterator::new(root);

    print!("Result -> ");
    print!("[");
    while iter.has_next() {
        let current = iter.next();

        if iter.has_next() {
            print!("{}, ", current);
        } else {
            print!("{}", current);
        }
    }
    println!("]");
}

struct BSTIterator {
    iter: Peekable<vec::IntoIter<i32>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut queue = Vec::new();
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, storage: &mut Vec<i32>) {
            if let Some(n) = root {
                let n_ref = n.borrow();

                dfs(n_ref.left.clone(), storage);
                storage.push(n_ref.val);
                dfs(n_ref.right.clone(), storage);
            }
        }
        dfs(root, &mut queue);

        BSTIterator {
            iter: queue.into_iter().peekable(),
        }
    }

    fn next(&mut self) -> i32 {
        self.iter.next().unwrap()
    }

    fn has_next(&mut self) -> bool {
        self.iter.peek().is_some()
    }
}

struct BSTIteratorStack {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIteratorStack {
    fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();

        while let Some(n) = root.take() {
            stack.push(n.clone());
            root = n.borrow().left.clone();
        }

        Self { stack }
    }

    fn next(&mut self) -> i32 {
        let next = self.stack.pop().unwrap();
        let next_ref = next.borrow();

        let mut current = next_ref.right.clone();
        while let Some(n) = current.take() {
            self.stack.push(n.clone());
            current = n.borrow().left.clone();
        }

        next_ref.val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}
