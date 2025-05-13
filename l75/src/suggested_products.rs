///
/// # [1268. Search Suggestions System](https://leetcode.com/problems/search-suggestions-system)
///
use std::collections::BTreeMap;

#[allow(unused_variables)]
fn main() {
    let (products, search_word) = (
        vec![
            "mobile".to_string(),
            "mouse".to_string(),
            "moneypot".to_string(),
            "monitor".to_string(),
            "mousepad".to_string(),
        ],
        "mouse".to_string(),
    );

    let (products, search_word) = (vec!["havana".to_string()], "havana".to_string());

    println!("Result -> {:?}", suggested_products(products, search_word));
}

#[derive(Default, Debug)]
struct TrieNode {
    is_end: bool,
    children: BTreeMap<char, Self>,
}

impl TrieNode {
    fn new(population: &[String]) -> Self {
        let mut head = TrieNode::default();

        for w in population {
            let mut tail = &mut head;

            for c in w.chars() {
                tail = tail.children.entry(c).or_default();
            }

            tail.is_end = true;
        }

        head
    }

    fn suggest(&self, target: &str, max: usize) -> Vec<String> {
        let mut suggest_head = self;
        for c in target.chars() {
            if let Some(node) = suggest_head.children.get(&c) {
                suggest_head = node;
            } else {
                return Vec::new();
            }
        }

        let mut result = Vec::new();

        let mut stack = vec![(target.to_string(), suggest_head)];
        while !stack.is_empty() && result.len() < max {
            let (prefix, node) = stack.pop().unwrap();

            if node.is_end {
                result.push(prefix.clone());
            }

            for (&c, node) in node.children.iter().rev() {
                let mut path_prefix = prefix.clone();
                path_prefix.push(c);

                stack.push((path_prefix, node));
            }
        }

        result
    }
}

fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    const MAX_SUGGESTION: usize = 3;

    let trie = TrieNode::new(&products);

    let mut result = Vec::new();
    for i in 0..search_word.len() {
        let suggestion = trie.suggest(&search_word[0..=i], MAX_SUGGESTION);
        result.push(suggestion);
    }

    result
}
