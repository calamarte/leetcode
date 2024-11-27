use core::str;
use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
    os::unix::raw::uid_t,
    usize,
};

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let s = "barfoothefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];

    // let s = "barfoofoobarthefoobarman".to_string();
    // let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
    //
    // let s = "wordgoodgoodgoodbestword".to_string();
    // let words = vec![
    //     "word".to_string(),
    //     "good".to_string(),
    //     "best".to_string(),
    //     "word".to_string(),
    // ];
    //
    // let s = "lingmindraboofooowingdingbarrwingmonkeypoundcake".to_string();
    // let words = vec![
    //     "fooo".to_string(),
    //     "barr".to_string(),
    //     "wing".to_string(),
    //     "ding".to_string(),
    //     "wing".to_string(),
    // ];
    //
    // let s = "wordgoodgoodgoodbestword".to_string();
    // let words = vec![
    //     "word".to_string(),
    //     "good".to_string(),
    //     "best".to_string(),
    //     "good".to_string(),
    // ];

    // let s = "ababababab".to_string();
    // let words = vec!["ababa".to_string(), "babab".to_string()];
    //
    // let s = "aaaaaaaaaaaaaa".to_string();
    // let words = vec!["aa".to_string(), "aa".to_string()];

    let result = find_substring(s, words);

    println!("Result -> {result:?}");
}

// NOTE: Not working is too slow posible solution using word len to skip
fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let word_len = words[0].len();
    let total_len = word_len * words.len();

    if total_len > s.len() {
        return Vec::new();
    }

    let words_map = prepare_map(&words);
    let mut result = Vec::with_capacity(s.len() / 2);

    println!("{:?}", prepare_map(&words));

    let mut used: HashMap<&str, i32> = HashMap::with_capacity(words_map.len());
    for (i, window) in s.as_bytes().windows(total_len).enumerate() {
        let is_valid = window
            .chunks_exact(word_len)
            .map(|cc| str::from_utf8(cc).unwrap()) // PERF:
            .all(|w| match words_map.get(w) {
                Some(&count) => {
                    let used_count = used.entry(w).or_default();
                    if *used_count >= count {
                        return false;
                    }

                    *used_count += 1;
                    true
                }
                _ => false,
            });

        println!("{:?} -> {is_valid}", str::from_utf8(window).unwrap());

        if is_valid {
            result.push(i as i32);
        }

        used.clear();
    }

    result
}

fn prepare_map(worlds: &[String]) -> HashMap<&str, i32> {
    let mut map: HashMap<&str, _> = HashMap::with_capacity(worlds.len());

    worlds.iter().for_each(|s| *map.entry(s).or_default() += 1);

    map
}
