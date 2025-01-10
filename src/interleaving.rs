#![allow(dead_code)]

#[allow(unused_variables)]
fn main() {
    let (s1, s2, s3) = (
        "aabcc".to_string(),
        "dbbca".to_string(),
        "aadbbcbcac".to_string(),
    );

    let (s1, s2, s3) = (
        "aabc".to_string(),
        "abad".to_string(),
        "aabadabc".to_string(),
    );

    println!("Result -> {}", is_interleave(s1, s2, s3));
}

/// Time complexity: $$O(2^n)$$
/// Space complexity: $$O(2^n)$$
fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    fn dfs(index: isize, target: &[u8], mut bank1: Vec<u8>, mut bank2: Vec<u8>) -> bool {
        if index < 0 {
            return true;
        }

        let v = target[index as usize];

        match (bank1.last(), bank2.last()) {
            (Some(&l1), None) if l1 == v => {
                bank1.pop();
            }
            (None, Some(&l2)) if l2 == v => {
                bank2.pop();
            }
            (Some(&l1), Some(&l2)) if l1 == v && l2 == v => {
                let (mut bank1_clone, bank2_clone) = (bank1.clone(), bank2.clone());
                bank1_clone.pop();

                if dfs(index - 1, target, bank1_clone, bank2_clone) {
                    return true;
                }

                bank2.pop();
                return dfs(index - 1, target, bank1, bank2);
            }
            (Some(&l1), _) if l1 == v => {
                bank1.pop();
            }
            (_, Some(&l2)) if l2 == v => {
                bank2.pop();
            }
            (_, _) => return false,
        }

        dfs(index - 1, target, bank1, bank2)
    }

    dfs(
        s3.len() as isize - 1,
        s3.as_bytes(),
        s1.as_bytes().to_vec(),
        s2.as_bytes().to_vec(),
    )
}

/// # Bottom-up
/// Time complexity: $$O(n \cdot m)$$
/// Space complexity: $$O(n \cdot m)$$
fn is_interleave_up(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let s3 = s3.as_bytes();

    let mut cache = vec![vec![false; s2.len() + 1]; s1.len() + 1];
    cache[s1.len()][s2.len()] = true;

    for i in (0..=s1.len()).rev() {
        for j in (0..=s2.len()).rev() {
            if i < s1.len() && s1[i] == s3[i + j] && cache[i + 1][j] {
                cache[i][j] = true;
            }

            if j < s2.len() && s2[j] == s3[i + j] && cache[i][j + 1] {
                cache[i][j] = true;
            }
        }
    }

    cache[0][0]
}
