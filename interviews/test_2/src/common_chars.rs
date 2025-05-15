use std::collections::HashMap;

fn main() {
    let words = vec![
        "bella".to_string(),
        "label".to_string(),
        "roller".to_string(),
    ];

    println!("Result -> {:?}", common_chars(words));
}

/// - Patterns: HashMap
/// - Time complexity: $$O(n \cdot m)$$
/// - Space complexity: $$O(k \cdot n + m)$$
fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut map: HashMap<char, Vec<i32>> = HashMap::new();
    let len = words.len();

    for (i, w) in words.into_iter().enumerate() {
        for c in w.chars() {
            map.entry(c).or_insert(vec![0; len])[i] += 1;
        }
    }

    let mut result = Vec::new();
    for (k, v) in map {
        let times = v.into_iter().min().unwrap();

        for _ in 0..times {
            result.push(k.to_string());
        }
    }

    result
}
