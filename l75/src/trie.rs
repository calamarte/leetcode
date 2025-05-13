#![allow(dead_code)]

///
/// # [208. Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree)
///
use std::{cell::RefCell, rc::Rc};

fn main() {
    todo!()
}

#[derive(Default)]
struct TrieNode {
    value: char,
    is_end: bool,
    children: Vec<Rc<RefCell<TrieNode>>>,
}

impl TrieNode {
    fn new(value: char) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    fn find_child(&self, target: char) -> Option<Rc<RefCell<TrieNode>>> {
        for node in &self.children {
            let node_ref = node.borrow();

            if target == node_ref.value {
                return Some(node.clone());
            }
        }

        None
    }
}

#[derive(Default)]
struct Trie {
    head: Rc<RefCell<TrieNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut tail = self.head.clone();

        for c in word.chars() {
            let current = tail;
            let mut current_mut = current.borrow_mut();

            tail = if let Some(child) = current_mut.find_child(c) {
                child
            } else {
                let new_child = Rc::new(RefCell::new(TrieNode::new(c)));
                current_mut.children.push(new_child.clone());

                new_child
            };
        }

        let mut tail_mut = tail.borrow_mut();
        tail_mut.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut tail = self.head.clone();

        for c in word.chars() {
            let current = tail.clone();
            let current_ref = current.borrow();

            if let Some(child) = current_ref.find_child(c) {
                tail = child;
            } else {
                return false;
            }
        }

        let tail_ref = tail.borrow();
        tail_ref.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut tail = self.head.clone();

        for c in prefix.chars() {
            let current = tail.clone();
            let current_ref = current.borrow();

            if let Some(child) = current_ref.find_child(c) {
                tail = child;
            } else {
                return false;
            }
        }

        true
    }
}
