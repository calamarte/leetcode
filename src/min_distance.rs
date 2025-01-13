fn main() {
    let (word1, word2) = ("horse".to_string(), "ros".to_string());

    println!("Result -> {}", min_distance(word1, word2));
}

/// # Bottom-up
/// Time complexity: $$O(n \cdot m)$$
/// Space complexity: $$O(n \cdot m)$$
fn min_distance(word1: String, word2: String) -> i32 {
    const DIRECTIONS: [(usize, usize); 3] = [(0, 1), (1, 0), (1, 1)];

    let mut cache = vec![vec![i32::MAX; word2.len() + 1]; word1.len() + 1];

    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();

    for j in 0..=word2.len() {
        cache[word1.len()][j] = (word2.len() - j) as i32;
    }

    #[allow(clippy::needless_range_loop)]
    for i in 0..=word1.len() {
        cache[i][word2.len()] = (word1.len() - i) as i32;
    }

    for i in (0..word1.len()).rev() {
        for j in (0..word2.len()).rev() {
            if word1[i] == word2[j] {
                cache[i][j] = cache[i + 1][j + 1];
                continue;
            }

            cache[i][j] = 1 + DIRECTIONS
                .iter()
                .map(|&(dy, dx)| cache[i + dy][j + dx])
                .min()
                .unwrap();
        }
    }

    cache[0][0]
}
