use std::{i32, thread::current};

fn main() {
    let ratings = vec![1, 0, 2];
    let ratings = vec![1, 2, 2];
    let ratings = vec![1, 3, 2, 2, 1]; // Expected result 7
                                       // let ratings = vec![1, 2, 87, 87, 87, 2, 1]; // Expect result 13
                                       // let ratings = vec![5, 4, 3, 5, 6, 2];

    let result = candy(ratings);

    println!("{result}");
}

fn candy(mut ratings: Vec<i32>) -> i32 {
    if ratings.len() == 1 {
        return 1;
    }

    let mut candys = vec![1; ratings.len()];

    for i in 1..ratings.len() {
        let left = ratings[i - 1];
        let right = ratings[i];

        if right > left {
            candys[i] = candys[i - 1] + 1;
        }
    }

    for i in (0..ratings.len() - 1).rev() {
        let left = ratings[i + 1];
        let right = ratings[i];

        if right > left {
            candys[i] = i32::max(candys[i], candys[i + 1] + 1);
        }
    }

    candys.into_iter().sum()
}
