fn main() {
    let result = remove_element(vec![3, 2, 2, 3].as_mut(), 3);
    println!("{result}")
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&v| v != val);
    nums.len() as i32
}
