#![allow(dead_code)]

use std::collections::VecDeque;

fn main() {}

const PING_TIME: i32 = 3000;

#[derive(Default)]
struct RecentCounter {
    times: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self::default()
    }

    /// - Patterns: VecDeque
    /// - Time comlexity: $$O(1)$$
    /// - Space comlexity: $$O(1)$$
    fn ping(&mut self, t: i32) -> i32 {
        self.times.push_back(t);

        while *self.times.front().expect("Never empty!") < t - PING_TIME {
            self.times.pop_front();
        }

        self.times.len() as i32
    }
}
