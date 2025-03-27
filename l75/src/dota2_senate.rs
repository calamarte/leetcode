use std::collections::VecDeque;

#[allow(unused_variables)]
fn main() {
    let senate = String::from("RD");
    let senate = String::from("RDD");
    let senate = String::from("RRR");

    println!("Result -> {}", predict_party_victory(senate));
}

/// - Patterns: VecDeque, greedy
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn predict_party_victory(senate: String) -> String {
    let mut senate: VecDeque<char> = senate.chars().collect();

    let mut radiant_counter = 0;
    let mut dire_counter = 0;
    while radiant_counter != senate.len() && dire_counter != senate.len() {
        let curr = senate.pop_front().expect("Never empty!");

        if curr == 'R' {
            if dire_counter > 0 {
                dire_counter -= 1;
                continue;
            }

            radiant_counter += 1;
            senate.push_back(curr);
        } else {
            if radiant_counter > 0 {
                radiant_counter -= 1;
                continue;
            }

            dire_counter += 1;
            senate.push_back(curr);
        }
    }

    if senate.pop_front().unwrap() == 'R' {
        "Radiant"
    } else {
        "Dire"
    }
    .to_string()
}
