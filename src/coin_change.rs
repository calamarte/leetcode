#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};

#[allow(unused_variables)]
fn main() {
    let (coins, amount) = (vec![1, 2, 5], 11);
    let (coins, amount) = (vec![1, 2, 5], 100);
    // let (coins, amount) = (vec![2], 3);
    // let (coins, amount) = (vec![1], 0);
    // let (coins, amount) = (vec![1], 1);

    println!("Result -> {}", coin_change_up(coins, amount));
}

/// # Top-down
/// Time complexity: $$O(c \cdot a)$$
/// Space complexity: $$O(a)$$
fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut visited = HashSet::new();
    let mut deq = VecDeque::new();

    deq.push_back((0, 0));

    while let Some((steps, a)) = deq.pop_front() {
        if visited.contains(&a) || a > amount {
            continue;
        }

        if a == amount {
            return steps;
        }

        visited.insert(a);

        for &c in &coins {
            deq.push_back((steps + 1, a + c));
        }
    }

    -1
}

/// # Bottom-up
/// Time complexity: $$O(c \cdot a)$$
/// Space complexity: $$O(a)$$
fn coin_change_up(coins: Vec<i32>, amount: i32) -> i32 {
    let cache: &mut [i32] = &mut vec![i32::MAX; amount as usize + 1];
    cache[0] = 0;

    for a in 1..amount + 1 {
        for &c in &coins {
            if a - c >= 0 {
                cache[a as usize] = cache[a as usize].min(1 + cache[a as usize - c as usize]);
            }
        }
    }

    if cache[amount as usize] != amount + 1 {
        cache[amount as usize]
    } else {
        -1
    }
}
