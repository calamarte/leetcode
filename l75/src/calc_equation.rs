use std::collections::{HashMap, HashSet};

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

/// - Patterns: DFS
/// - Time complexity: $$O(n \cdot (E + V))$$
/// - Time complexity: $$O(E + V)$$
// NOTE: At least I could understand the problem this time :)
fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    fn dfs(
        src: &str,
        target: &str,
        relations: &HashMap<String, Vec<(f64, String)>>,
        visited: &mut HashSet<String>,
    ) -> f64 {
        if src == target {
            return 1.0;
        }

        visited.insert(src.to_string());

        if let Some(n) = relations.get(src) {
            for &(val, ref next) in n {
                if !visited.contains(next) {
                    let result = dfs(next, target, relations, visited);
                    if result != -1.0 {
                        return val * result;
                    }
                }
            }
        }

        -1.0
    }

    let map: HashMap<String, Vec<(f64, String)>> =
        equations
            .into_iter()
            .zip(values)
            .fold(HashMap::new(), |mut acc, (div, res)| {
                acc.entry(div[0].clone())
                    .or_default()
                    .push((res, div[1].clone()));

                acc.entry(div[1].clone())
                    .or_default()
                    .push((1.0 / res, div[0].clone()));

                acc
            });

    println!("Graph -> {map:?}");
    queries
        .into_iter()
        .map(|q| {
            let mut visited = HashSet::new();
            if map.contains_key(&q[0]) && map.contains_key(&q[1]) {
                dfs(&q[0], &q[1], &map, &mut visited)
            } else {
                -1.0
            }
        })
        .collect()
}
