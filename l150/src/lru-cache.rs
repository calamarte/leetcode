use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, LinkedList, VecDeque},
    rc::Rc,
};

fn main() {
    println!("Hello world!");
}

#[derive(Default)]
struct LRUCache {
    cache: HashMap<i32, i32>,
    keys: VecDeque<i32>,
    capacity: usize,
    total: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            ..Default::default()
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.cache.get(&key) {
            Some(&val) => {
                self.update_key(key);
                val
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.cache.insert(key, value) {
            Some(_) => self.update_key(key),
            None => {
                self.keys.push_front(key);
                self.total += 1;
            }
        }

        if self.total > self.capacity {
            if let Some(k) = self.keys.pop_back() {
                self.cache.remove(&k);
                self.total -= 1;
            }
        }
    }

    fn update_key(&mut self, key: i32) {
        let idx = self.keys.iter().position(|&k| k == key).unwrap();
        self.keys.remove(idx);
        self.keys.push_front(key);
    }
}
