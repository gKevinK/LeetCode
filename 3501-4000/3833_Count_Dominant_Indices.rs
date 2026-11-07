impl Solution {
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        let mut sum = nums[n - 1];
        for i in (0..n - 1).rev() {
            if nums[i] * (n - 1 - i) as i32 > sum {
                res += 1;
            }
            sum += nums[i];
        }
        res
    }
}