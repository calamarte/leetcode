#![allow(dead_code)]

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    // let nums = vec![5, 4, 3, 2, 1];

    println!("Result -> {}", increasing_triplet_op(nums));
}

/// - Patterns: greedy
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut triplet = Vec::with_capacity(3);

    for v in nums {
        for i in 0..3 {
            if let Some(tv) = triplet.get_mut(i) {
                if v <= *tv {
                    *tv = v;
                    break;
                }
            } else {
                triplet.push(v);
                break;
            }
        }

        if triplet.len() == 3 {
            return true;
        }
    }

    false
}

/// - Patterns: greedy
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn increasing_triplet_op(nums: Vec<i32>) -> bool {
    let (mut first, mut sec) = (i32::MAX, i32::MAX);

    for v in nums {
        if v <= first {
            first = v;
            continue;
        }

        if v <= sec {
            sec = v;
            continue;
        }

        return true;
    }

    false
}
