use std::collections::{HashMap, HashSet};

fn main() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];

    let words = vec!["eat".to_string(), "oath".to_string(), "pepino".to_string()];

    println!("Result -> {:?}", find_words(board, words));
}

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, word: &str) {
        let mut current = self;
        for c in word.chars() {
            current = current.children.entry(c).or_default();
        }

        current.is_end = true;
    }
}

#[derive(Default)]
struct Finder {
    board: Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
    result: HashSet<String>,
}

impl Finder {
    fn new(board: Vec<Vec<char>>) -> Self {
        Self {
            board,
            ..Default::default()
        }
    }

    // FIXME: Didn't work is too slow, leetcode wins this round
    fn find(&mut self, words: Vec<String>) -> Vec<String> {
        let trie = words.iter().fold(TrieNode::new(), |mut trie, w| {
            trie.add(w);
            trie
        });

        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                self.dfs((y, x), &trie, String::new());
            }
        }

        self.result.clone().into_iter().collect()
    }

    fn dfs(&mut self, cords: (usize, usize), mut node: &TrieNode, mut word: String) {
        let (h, w) = (self.board.len(), self.board[0].len());
        let (y, x) = cords;

        if y == h
            || x == w
            || self.visited.contains(&cords)
            || !node.children.contains_key(&self.board[y][x])
        {
            return;
        }

        self.visited.insert(cords);

        let c = self.board[y][x];
        node = node.children.get(&c).unwrap();
        word.push(c);

        if node.is_end {
            self.result.insert(word.clone());
        }

        if let Some(y) = y.checked_sub(1) {
            self.dfs((y, x), node, word.clone());
        }
        self.dfs((y + 1, x), node, word.clone());

        if let Some(x) = x.checked_sub(1) {
            self.dfs((y, x), node, word.clone());
        }
        self.dfs((y, x + 1), node, word.clone());

        self.visited.remove(&cords);
    }
}

fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    Finder::new(board).find(words)
}

/// WARN: Copied solution from [leetcode](https://leetcode.com/problems/word-search-ii/solutions/3302660/rust-the-fastest-63ms-custom-dictonary-implementation-with-o-1-for-check-next-char)
pub fn find_words_leetcode(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    fn sub(
        board: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        dict: &mut Vec<(usize, bool)>,
        mut dpos: usize,
        t: &mut String,
        res: &mut Vec<String>,
    ) {
        let c = board[y][x];
        let index = dpos + (c as u8 - b'a') as usize;
        if (!dict[index].1) && (dict[index].0 == 0) {
            return;
        }
        const VISITED: char = '#';
        board[y][x] = VISITED;
        t.push(c);

        if dict[index].1 {
            //check if word present
            dict[index].1 = false; //delete word flag
            res.push(t.clone()); //push temporary string to result
        }
        if dict[index].0 != 0 {
            dpos = dict[index].0;
            if x < board[0].len() - 1 && board[y][x + 1] != VISITED {
                sub(board, x + 1, y, dict, dpos, t, res);
            };
            if x > 0 && board[y][x - 1] != VISITED {
                sub(board, x - 1, y, dict, dpos, t, res);
            };
            if y < board.len() - 1 && board[y + 1][x] != VISITED {
                sub(board, x, y + 1, dict, dpos, t, res);
            };
            if y > 0 && board[y - 1][x] != VISITED {
                sub(board, x, y - 1, dict, dpos, t, res);
            };
        }
        board[y][x] = c;
        t.pop();
    }

    //first in pair - next index, second - word present flag
    //each node in the vector contains block of 26 pair for each latin alphabet char
    let mut dict = vec![(0usize, false); 26];
    for word in words {
        let mut dpos = 0;
        for c in word.as_bytes().iter().enumerate() {
            let index = dpos + (c.1 - b'a') as usize;
            if c.0 == word.len() - 1 {
                dict[index].1 = true; //mark word is present
            } else if dict[index].0 != 0 {
                // check if the next node is present
                dpos = dict[index].0;
            } else {
                // add new node
                dpos = dict.len();
                dict[index].0 = dict.len();
                dict.extend_from_slice(&[(0usize, false); 26]);
            }
        }
    }

    let mut res = vec![];
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            sub(&mut board, x, y, &mut dict, 0, &mut String::new(), &mut res);
        }
    }

    res
}
