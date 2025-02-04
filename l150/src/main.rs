fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];

    merge(&mut nums1, 3, &mut nums2, 3);
    println!("{nums1:?}");
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    nums1.truncate(m as usize);
    nums1.append(nums2);
    nums1.sort();
}
