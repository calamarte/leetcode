use std::collections::HashMap;

fn main() {
    let (s, t) = (String::from("ADOBECODEBANC"), String::from("ABC"));

    let result = min_window(s, t);

    println!("Result -> {result}");
}

fn min_window(s: String, t: String) -> String {
    if t.is_empty() {
        return t;
    }

    let s: Vec<char> = s.chars().collect();
    let t: HashMap<char, i32> = t.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_default() += 1;
        map
    });

    let need = t.len();
    let mut have = 0;
    let mut left = 0;

    let mut min_subset: Option<&[char]> = None;
    let mut window_matches: HashMap<char, i32> = HashMap::new();
    for right in 0..s.len() {
        // Update current values
        let current = s[right];
        let current_count = window_matches.entry(current).or_default();

        *current_count += 1;

        // Update have state
        match t.get(&current) {
            Some(tc) if tc == current_count => have += 1,
            _ => (),
        }

        // Check for valid substring
        while have == need {
            // Update min substring
            match min_subset {
                Some(ref mut sub) if right - left + 1 < sub.len() => *sub = &s[left..=right],
                None => min_subset = Some(&s[left..=right]),
                _ => (),
            };

            // Move left pointer
            window_matches
                .entry(s[left])
                .and_modify(|entry| *entry -= 1);

            match (t.get(&s[left]), window_matches.get(&s[left])) {
                (Some(tc), Some(w)) if tc > w => have -= 1,
                _ => (),
            }

            left += 1;
        }
    }

    match min_subset {
        Some(sub) => String::from_iter(sub),
        _ => String::new(),
    }
}
