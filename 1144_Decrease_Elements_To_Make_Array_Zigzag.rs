impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        use std::cmp::min;
        let mut r = [0, 0];
        for i in 0..nums.len() {
            let mut t = nums[i];
            if i > 0 {
                t = min(t, nums[i - 1] - 1);
            }
            if i + 1 < nums.len() {
                t = min(t, nums[i + 1] - 1);
            }
            r[i % 2] += nums[i] - t;
        }
        min(r[0], r[1])
    }
}