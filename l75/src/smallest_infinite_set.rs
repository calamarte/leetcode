use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let mut small_set = SmallestInfiniteSet::new();
    small_set.add_back(2);
    small_set.pop_smallest();
    small_set.pop_smallest();
    small_set.pop_smallest();
    small_set.add_back(1);
    small_set.pop_smallest();
    small_set.pop_smallest();
    small_set.pop_smallest();
}

#[derive(Default)]
struct SmallestInfiniteSet {
    min_heap: BinaryHeap<Reverse<i32>>,
    removed: HashSet<i32>,
}

/// - Patterns: Heap, HashSet
/// - Space complexity: $$O(n)$$
impl SmallestInfiniteSet {
    fn new() -> Self {
        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse(1));

        Self {
            min_heap,
            ..Default::default()
        }
    }

    /// - Time complexity: $$O(log n)$$
    fn pop_smallest(&mut self) -> i32 {
        let value = self.min_heap.pop().expect("Never empty!").0;

        self.removed.insert(value);

        if self.min_heap.is_empty() {
            let mut new_value = value + 1;
            while self.removed.contains(&new_value) {
                new_value += 1;
            }

            self.min_heap.push(Reverse(new_value));
        }

        value
    }

    /// - Time complexity: $$O(log n)$$
    fn add_back(&mut self, num: i32) {
        if self.removed.remove(&num) {
            self.min_heap.push(Reverse(num));
        }
    }
}
