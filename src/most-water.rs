#[allow(unused_variables)]
#[allow(clippy::useless_vec)]
fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let height = vec![8, 7, 2, 1];
    let result = max_area(height);

    println!("{result}");
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0usize;
    let mut right = height.len() - 1;

    while left < right {
        let h = height[left].min(height[right]);
        let w = (right - left) as i32;

        max_area = max_area.max(h * w);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}
