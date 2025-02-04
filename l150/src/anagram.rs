use std::collections::HashMap;

fn main() {
    let result = is_anagram_map("anagram".to_string(), "nagaram".to_string());

    println!("{result}");
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut sv: Vec<char> = s.chars().collect();
    let mut tv: Vec<char> = t.chars().collect();

    sv.sort();
    tv.sort();

    sv == tv
}

fn is_anagram_map(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map: HashMap<char, isize> = HashMap::new();
    for (chs, cht) in s.chars().zip(t.chars()) {
        *map.entry(chs).or_default() += 1;
        *map.entry(cht).or_default() -= 1;
    }

    map.values().all(|&v| v == 0)
}
