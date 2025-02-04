use std::collections::{HashSet, VecDeque};

#[allow(unused_variables)]
fn main() {
    let (begin_word, end_word, word_list) = (
        "hit".to_string(),
        "cog".to_string(),
        vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
            "cog".to_string(),
        ],
    );

    let (begin_word, end_word, word_list) = (
        "hit".to_string(),
        "cog".to_string(),
        vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
        ],
    );

    println!(
        "Result -> {}",
        ladder_length(begin_word, end_word, word_list)
    );
}

trait Transform {
    fn is_valid_transform(&self, target: &str) -> bool;
}

impl Transform for str {
    fn is_valid_transform(&self, target: &str) -> bool {
        let origin = self.bytes();
        let target = target.bytes();

        if origin.len() != target.len() {
            return false;
        }

        origin.zip(target).filter(|(a, b)| a != b).count() == 1
    }
}

/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n^2)$$
fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut visited = HashSet::new();
    let mut deq = VecDeque::new();

    deq.push_front((&begin_word, 1));

    while let Some((word, step)) = deq.pop_back() {
        for next_word in &word_list {
            if !word.is_valid_transform(next_word) {
                continue;
            }

            if next_word == &end_word {
                return step + 1;
            }

            if visited.insert(next_word) {
                deq.push_front((next_word, step + 1));
            }
        }
    }

    0
}
