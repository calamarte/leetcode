use std::collections::HashMap;

fn main() {
    println!("Hello trie!");

    let mut trie = Trie::new();

    trie.insert("pepino".to_string());

    println!("Search -> {}", trie.search("pep".to_string()));
    println!("Starts -> {}", trie.starts_with("pep".to_string()));

    println!("{trie:?}");
}

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_default();
        }

        current.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut current = &self.root;

        for c in word.chars() {
            if let Some(tn) = current.children.get(&c) {
                current = tn;
            } else {
                return false;
            }
        }

        current.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;

        for c in prefix.chars() {
            if let Some(tn) = current.children.get(&c) {
                current = tn;
            } else {
                return false;
            }
        }

        true
    }
}

// Custom impl
// #[derive(Debug, Default)]
// struct TrieNode {
//     next: Rc<RefCell<HashMap<char, TrieNode>>>,
// }
//
// #[derive(Debug)]
// struct Trie {
//     tree: Rc<RefCell<HashMap<char, TrieNode>>>,
//     words: HashSet<String>,
// }
//
// impl Trie {
//     fn new() -> Self {
//         Trie {
//             tree: Rc::new(RefCell::new(HashMap::new())),
//             words: HashSet::new(),
//         }
//     }
//
//     fn insert(&mut self, word: String) {
//         if word.is_empty() {
//             return;
//         }
//
//         let mut words = self.tree.clone();
//
//         for c in word.chars() {
//             let w = words.clone();
//             let mut w_mut = w.borrow_mut();
//
//             words = w_mut.entry(c).or_default().next.clone();
//         }
//
//         self.words.insert(word);
//     }
//
//     fn search(&self, word: String) -> bool {
//         self.words.contains(&word)
//     }
//
//     fn starts_with(&self, prefix: String) -> bool {
//         let mut words = self.tree.clone();
//
//         for c in prefix.chars() {
//             let w = words.clone();
//             let w_ref = w.borrow();
//
//             if let Some(n) = w_ref.get(&c) {
//                 words = n.next.clone();
//             } else {
//                 return false;
//             }
//         }
//
//         true
//     }
// }
