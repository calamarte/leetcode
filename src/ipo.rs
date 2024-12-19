#![allow(dead_code)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[allow(unused_variables)]
fn main() {
    let (k, w) = (2, 0);
    let (profits, capital) = (vec![1, 2, 3], vec![0, 1, 1]);

    let (k, w) = (10, 0);
    let (profits, capital) = (vec![1, 2, 3], vec![0, 1, 2]);

    let (k, w) = (2, 0);
    let (profits, capital) = (vec![1, 2, 3], vec![0, 9, 10]);

    println!(
        "Result -> {}",
        find_maximized_capital_heap(k, w, profits, capital)
    );
}

// INFO: Works well but too slow too much iterations and is overcomplicated
fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let balance: Vec<(usize, i32, i32)> = profits
        .into_iter()
        .zip(capital)
        .enumerate()
        .map(|(i, (p, c))| (i, p, c))
        .collect();

    let mut visited = HashSet::new();

    for _ in 0..(balance.len().min(k as usize)) {
        let candidate = balance
            .iter()
            .filter(|&&(i, _, c)| c <= w && !visited.contains(&i))
            .max_by_key(|&(_, p, _)| p);

        if let Some(&(i, p, _)) = candidate {
            w += p;
            visited.insert(i);
        } else {
            return w;
        }
    }

    w
}

// Heap aproach
fn find_maximized_capital_heap(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut max_profit = BinaryHeap::with_capacity(profits.len());
    let mut min_capital: BinaryHeap<_> = capital.into_iter().zip(profits).map(Reverse).collect();

    for _ in 0..k {
        while let Some(&Reverse((c, p))) = min_capital.peek() {
            if c > w {
                break;
            }

            min_capital.pop();
            max_profit.push(p);
        }

        if let Some(v) = max_profit.pop() {
            w += v;
        } else {
            break;
        }
    }

    w
}
