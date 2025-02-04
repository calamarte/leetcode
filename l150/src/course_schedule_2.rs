use std::collections::{HashMap, HashSet};

#[allow(unused_variables)]
fn main() {
    let (num_courses, prereq) = (2, vec![vec![1, 0]]);
    let (num_courses, prereq) = (4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);

    println!("Result -> {:?}", find_order(num_courses, prereq));
}

/// Topological Sort [wikipedia](https://en.wikipedia.org/wiki/Topological_sorting)
/// Time complexity: $$O(V + E)$$
/// Space complexity: $$O(V + E)$$
fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let requisites: HashMap<_, Vec<_>> =
        prerequisites
            .into_iter()
            .fold(HashMap::new(), |mut acc, v| {
                acc.entry(v[0]).or_default().push(v[1]);
                acc
            });

    let (mut visit, mut cycle) = (HashSet::new(), HashSet::new());
    let mut result = Vec::new();

    for c in 0..num_courses {
        if !backtrack(c, &requisites, &mut cycle, &mut visit, &mut result) {
            return Vec::new();
        }
    }

    result
}

fn backtrack(
    c: i32,
    req_map: &HashMap<i32, Vec<i32>>,
    cycle: &mut HashSet<i32>,
    visit: &mut HashSet<i32>,
    storage: &mut Vec<i32>,
) -> bool {
    if cycle.contains(&c) {
        return false;
    }

    if visit.contains(&c) {
        return true;
    }

    cycle.insert(c);
    if let Some(reqs) = req_map.get(&c) {
        for &r in reqs {
            if !backtrack(r, req_map, cycle, visit, storage) {
                return false;
            }
        }
    }
    cycle.remove(&c);
    visit.insert(c);
    storage.push(c);

    true
}
