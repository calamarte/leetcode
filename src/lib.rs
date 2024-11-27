use std::{cell::RefCell, collections::VecDeque, iter::successors, rc::Rc};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(list: &[i32]) -> Option<Box<ListNode>> {
        if list.is_empty() {
            return None;
        }

        if list.len() == 1 {
            return Some(Box::new(ListNode::new(list[0])));
        }

        let mut dummy = ListNode::new(-1);
        list.iter().fold(&mut dummy, |head, &v| {
            let node = ListNode::new(v);
            head.next = Some(Box::new(node));
            head.next.as_mut().unwrap()
        });

        dummy.next
    }
}

pub fn sexy_format(mut to_print: Option<Box<ListNode>>) -> String {
    let mut vec = Vec::new();
    while let Some(n) = to_print {
        vec.push(n.val);
        to_print = n.next
    }

    format!("{vec:?}")
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            ..Default::default()
        }
    }

    pub fn as_link(self) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(self)))
    }

    pub fn as_link_unchecked(self) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(self))
    }

    pub fn from(list: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if list.is_empty() || list[0].is_none() {
            return None;
        }

        let root = TreeNode::new(list[0].unwrap()).as_link_unchecked();
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while i < list.len() {
            if let Some(node) = queue.pop_front() {
                if i < list.len() && list[i].is_some() {
                    let left_child = TreeNode::new(list[i]?).as_link_unchecked();
                    node.borrow_mut().left = Some(left_child.clone());
                    queue.push_back(left_child);
                }
                i += 1;

                if i < list.len() && list[i].is_some() {
                    let right_child = TreeNode::new(list[i]?).as_link_unchecked();
                    node.borrow_mut().right = Some(right_child.clone());
                    queue.push_back(right_child);
                }
                i += 1;
            }
        }

        Some(root)
    }
}

pub fn right_format(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let vec: Vec<_> = successors(root, |r| r.borrow().right.clone())
        .map(|n| n.borrow().val)
        .collect();

    format!("{vec:?}")
}
