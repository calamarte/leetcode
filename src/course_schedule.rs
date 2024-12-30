use std::collections::{HashMap, HashSet};

#[allow(unused_variables)]
fn main() {
    let (num_courses, prereq) = (2, vec![vec![1, 0]]);
    let (num_courses, prereq) = (2, vec![vec![1, 0], vec![0, 1]]);
    let (num_courses, prereq) = (5, vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]);
    let (num_courses, prereq) = (3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]);

    println!("Result -> {}", can_finish(num_courses, prereq));
}

/// Time Complexity: $$O(C + P)$$
/// Space Complexity: $$O(C + P)$$
fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut visited = HashSet::new();
    let mut requisites_map = prerequisites.into_iter().fold(
        HashMap::with_capacity(num_courses as usize),
        |mut acc: HashMap<i32, Vec<i32>>, v| {
            acc.entry(v[0]).or_default().push(v[1]);
            acc
        },
    );

    fn is_valid(c: i32, req: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>) -> bool {
        if visited.contains(&c) {
            return false;
        }

        if let Some(r) = req.get(&c) {
            visited.insert(c);

            if r.iter().any(|&rc| !is_valid(rc, req, visited)) {
                return false;
            }

            visited.remove(&c);
        }

        true
    }

    for i in 0..num_courses {
        if !is_valid(i, &requisites_map, &mut visited) {
            return false;
        }

        requisites_map.remove(&i);
    }

    true
}
