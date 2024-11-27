use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let (ransom_note, magazine) = ("a".to_string(), "b".to_string());
    let (ransom_note, magazine) = ("aa".to_string(), "ab".to_string());
    let (ransom_note, magazine) = ("aa".to_string(), "aab".to_string());

    println!("Result -> {}", can_construct(ransom_note, magazine));
}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut magazine: HashMap<char, i32> = magazine.chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|n| *n += 1).or_insert(1);
        acc
    });

    for c in ransom_note.chars() {
        match magazine.get_mut(&c) {
            Some(n) if *n > 0 => *n -= 1,
            _ => return false,
        }
    }

    true
}
