use std::collections::{BTreeMap, HashMap};

fn main() {
    let (arr1, arr2) = (
        vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
        vec![2, 1, 4, 3, 9, 6],
    );

    println!("Result -> {:?}", relative_sort_array(arr1, arr2));
}

fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len());

    let mut map = arr1.into_iter().fold(BTreeMap::new(), |mut acc, v| {
        acc.entry(v).and_modify(|c| *c += 1).or_insert(1);
        acc
    });

    for v in arr2 {
        for _ in 0..map.remove(&v).unwrap() {
            result.push(v);
        }
    }

    for (k, v) in map {
        for _ in 0..v {
            result.push(k);
        }
    }

    result
}
