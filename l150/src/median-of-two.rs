fn main() {
    #[rustfmt::skip]
    let (nums1, nums2) = (
        vec![1, 3],
        vec![2]
    );

    println!("Result -> {}", find_median_sorted_arrays(nums1, nums2));
}

// FIXME: Didn't work is too hard for now :(
fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    let total = (nums1.len() + nums2.len()) as i32;
    let half = total / 2;

    if nums1.len() < nums2.len() {
        (nums1, nums2) = (nums2, nums1);
    }

    let (mut left, mut right) = (0, (nums1.len() - 1) as i32);

    loop {
        let i = (left + right) / 2;
        let j = half - i - 2;

        #[rustfmt::skip]
        let a_left = if i >= 0 {nums1[i as usize]} else {i32::MIN};
        #[rustfmt::skip]
        let a_right = if (i +1) < nums1.len() as i32 {nums1[(i + 1) as usize]} else {i32::MAX};

        #[rustfmt::skip]
        let b_left = if j >= 0 { nums1[j as usize] } else { i32::MIN };
        #[rustfmt::skip]
        let b_right = if (j + 1) < nums2.len() as i32 { nums1[(j + 1) as usize]} else {i32::MAX};

        if a_left <= b_right && b_left <= a_right {
            if total % 2 == 0 {
                return a_right.min(b_right).into();
            }
            return (a_left.max(b_left) as f64 + a_right.min(b_right) as f64) / 2.0;
        } else if a_left > b_right {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }
}

// FIXME: Time complexity O((n + m) * log(n + m))
#[allow(dead_code)]
fn find_median_sorted_arrays_a(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    nums1.extend(nums2);
    nums1.sort();

    if nums1.len() == 1 {
        return nums1.pop().unwrap().into();
    }

    let middle = (nums1.len() - 1) / 2;

    if nums1.len() % 2 != 0 {
        nums1[middle].into()
    } else {
        let (a, b): (f64, f64) = (nums1[middle].into(), nums1[middle + 1].into());

        (a + b) / 2.0
    }
}
