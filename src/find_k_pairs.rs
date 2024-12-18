use std::{cmp::Reverse, collections::BinaryHeap, usize};

#[allow(unused_variables)]
fn main() {
    let (nums1, nums2, k) = (vec![1, 7, 11], vec![2, 4, 6], 3);
    let (nums1, nums2, k) = (vec![1, 1, 2], vec![1, 2, 3], 2);

    println!("Result -> {:?}", k_smallest_pairs(nums1, nums2, k));
}

fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut min_heap = nums1
        .iter()
        .enumerate()
        .fold(BinaryHeap::new(), |mut heap, (i, &v)| {
            heap.push((Reverse(v + nums2.first().unwrap()), i, 0usize));
            heap
        });

    while let Some((_, i, j)) = min_heap.pop() {
        result.push(vec![nums1[i], nums2[j]]);

        if result.len() == k as usize {
            break;
        }

        if j < nums2.len() - 1 {
            min_heap.push((Reverse(nums1[i] + nums2[j + 1]), i, j + 1));
        }
    }

    result
}
