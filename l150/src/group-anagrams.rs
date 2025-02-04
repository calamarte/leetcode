use std::collections::HashMap;

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];

    println!("Result -> {:?}", group_anagrams(strs));
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut grouped: HashMap<[u32; 26], Vec<String>> = HashMap::new();

    for word in strs {
        let mut count = [0_u32; 26];

        for c in word.chars() {
            count[c as usize - 'a' as usize] += 1;
        }

        grouped.entry(count).or_default().push(word);
    }

    grouped.into_values().collect()
}
