impl Solution {
    pub fn left_rigth_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        let mut sum_l = 0;
        let mut sum_r = 0;
        for i in 0..nums.len() {
            sum_r += nums[i];
        }
        for i in 0..nums.len() {
            sum_r -= nums[i];
            if i >= 1 {
                sum_l += nums[i - 1];
            }
            ans[i] = (sum_r - sum_l).abs();
        }
        ans
    }
}