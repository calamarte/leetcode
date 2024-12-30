use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let (equations, values, queries) = (
        vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
        ],
        vec![2.0, 3.0],
        vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["b".to_string(), "a".to_string()],
            vec!["a".to_string(), "e".to_string()],
            vec!["a".to_string(), "a".to_string()],
            vec!["x".to_string(), "x".to_string()],
        ],
    );

    println!("Result -> {:?}", calc_equation(equations, values, queries));
}

/// Time complexity: $$O(E + Q  \cdot E)$$
/// Space complexity: $$O(E + N)$$
/// > INFO: This problem was hard :( solution -> [here](https://www.youtube.com/watch?v=Uei1fwDoyKk)
fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let map = equations.iter().zip(values).fold(
        HashMap::new(),
        |mut map: HashMap<&str, Vec<(&str, _)>>, (eq, val)| {
            map.entry(&eq[0]).or_default().push((&eq[1], val));
            map.entry(&eq[1]).or_default().push((&eq[0], 1.0 / val));
            map
        },
    );

    fn bfs(src: &str, target: &str, eqs: &HashMap<&str, Vec<(&str, f64)>>) -> f64 {
        if !eqs.contains_key(src) || !eqs.contains_key(target) {
            return -1.0;
        }

        let mut deq = VecDeque::new();
        let mut visited = HashSet::new();

        deq.push_back((src, 1.0));
        visited.insert(src);

        while let Some((t, r)) = deq.pop_front() {
            if t == target {
                return r;
            }

            for &(nei, total) in eqs.get(t).unwrap() {
                if !visited.contains(nei) {
                    deq.push_back((nei, total * r));
                    visited.insert(nei);
                }
            }
        }

        -1.0
    }

    queries
        .into_iter()
        .map(|v| bfs(&v[0], &v[1], &map))
        .collect()
}
