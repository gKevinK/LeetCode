impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last = -k - 1;
        for (i, n) in nums.iter().enumerate() {
            if *n == 1 {
                if last + k >= i as i32 {
                    return false;
                }
                last = i as i32;
            }
        }
        true
    }
}