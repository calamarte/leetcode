#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![0, 1, 2, 4, 5, 7];
    let nums = vec![0, 2, 3, 4, 6, 8, 9];

    println!("Result -> {:?}", summary_ranges(nums));
}

fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut ranges: Vec<String> = Vec::new();
    let (mut start, mut end) = (nums[0], nums[0]);

    for n in nums.into_iter().skip(1) {
        if end + 1 == n {
            end = n;
            continue;
        }

        if start == end {
            ranges.push(start.to_string());
        } else {
            ranges.push(format!("{start}->{end}"));
        }

        (start, end) = (n, n)
    }

    if start == end {
        ranges.push(start.to_string());
    } else {
        ranges.push(format!("{start}->{end}"));
    }

    ranges
}
