fn main() {
    let citations = vec![3, 0, 6, 1, 5];
    let result = h_index(citations);

    println!("{result}");
}

fn h_index(citations: Vec<i32>) -> i32 {
    let mut citations = citations;
    citations.sort();
    citations.reverse();

    let mut current_rank = 0;
    for (rank, citation) in (1..).zip(citations.into_iter()) {
        if citation < rank {
            break;
        }

        current_rank = rank;
    }

    current_rank
}
