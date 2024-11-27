use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let (pattern, s) = ("abba".to_string(), "dog cat cat dog".to_string());
    let (pattern, s) = ("abba".to_string(), "dog cat cat fish".to_string());
    let (pattern, s) = ("aaa".to_string(), "aa aa aa aa".to_string());

    println!("Result -> {}", word_pattern(pattern, s));
}

fn word_pattern(pattern: String, s: String) -> bool {
    let s: Vec<&str> = s.split(" ").collect();

    if pattern.len() != s.len() {
        return false;
    }

    let mut pattern_map: HashMap<char, &str> = HashMap::new();
    let mut s_map: HashMap<&str, char> = HashMap::new();

    for (pc, sw) in pattern.chars().zip(s) {
        match (pattern_map.get(&pc), s_map.get(sw)) {
            (Some(&rpc), Some(&rsw)) if rpc == sw && rsw == pc => (),
            (None, None) => {
                pattern_map.insert(pc, sw);
                s_map.insert(sw, pc);
            }
            _ => return false,
        }
    }

    true
}
