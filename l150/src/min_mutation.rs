use std::collections::{HashSet, VecDeque};

#[allow(unused_variables)]
fn main() {
    let (start_gene, end_gene, bank) = (
        "AACCGGTT".to_string(),
        "AACCGGTA".to_string(),
        vec!["AACCGGTA".to_string()],
    );

    let (start_gene, end_gene, bank) = (
        "AACCGGTT".to_string(),
        "AAACGGTA".to_string(),
        vec![
            "AACCGGTA".to_string(),
            "AACCGCTA".to_string(),
            "AAACGGTA".to_string(),
        ],
    );

    let (start_gene, end_gene, bank) = (
        "AAAACCCC".to_string(),
        "CCCCCCCC".to_string(),
        vec![
            "AAAACCCA".to_string(),
            "AAACCCCA".to_string(),
            "AACCCCCA".to_string(),
            "AACCCCCC".to_string(),
            "ACCCCCCC".to_string(),
            "CCCCCCCC".to_string(),
            "AAACCCCC".to_string(),
            "AACCCCCC".to_string(),
        ],
    );

    println!("Result -> {}", min_mutation(start_gene, end_gene, bank));
}

trait Mutation {
    fn is_valid_mutation(&self, mutation: &str) -> bool;
}

impl Mutation for str {
    fn is_valid_mutation(&self, mutation: &str) -> bool {
        let origin = self.bytes();
        let mutation = mutation.bytes();

        if origin.len() != mutation.len() {
            return false;
        }

        origin.zip(mutation).filter(|(a, b)| a != b).count() == 1
    }
}

/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n^2)$$
fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
    if start_gene == end_gene {
        return 0;
    }

    let mut visited = HashSet::with_capacity(bank.len());
    let mut deq = VecDeque::with_capacity(bank.len());
    deq.push_front((&start_gene, 0));

    while let Some((gene, generation)) = deq.pop_back() {
        for next_gen in &bank {
            if !gene.is_valid_mutation(next_gen) {
                continue;
            }

            if next_gen == &end_gene {
                return generation + 1;
            }

            if visited.insert(next_gen) {
                deq.push_front((next_gen, generation + 1));
            }
        }
    }

    -1
}
