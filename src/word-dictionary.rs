use std::collections::HashMap;

fn main() {
    println!("Hello dictionary!");

    let mut dict = WordDictionary::new();
    dict.add_word("pepino".to_string());
    dict.add_word("pepsi".to_string());
    dict.add_word("dad".to_string());
    dict.add_word("mad".to_string());
    dict.add_word("mam".to_string());
    dict.add_word("at".to_string());

    println!("Search -> {}", dict.search(".a.".to_string()));
}

#[derive(Default, Debug)]
struct WordDictionary {
    children: HashMap<char, WordDictionary>,
    is_end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        let mut current = self;
        for c in word.chars() {
            current = current.children.entry(c).or_default();
        }

        current.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut current = self;
        for (i, c) in word.char_indices() {
            if c == '.' {
                let rest = &word[i + 1..];

                return current
                    .children
                    .values()
                    .any(|wd| wd.search(rest.to_string()));
            }

            if let Some(wd) = current.children.get(&c) {
                current = wd;
            } else {
                return false;
            }
        }

        current.is_end
    }
}
