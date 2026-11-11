impl Solution {
    pub fn rob(nums: Vec<i32>, colors: Vec<i32>) -> i64 {
        let mut dp0 = 0;
        let mut dp1 = 0;
        let n = nums.len();
        for i in 0..n {
            let max = dp0.max(dp1);
            if i == 0 || colors[i - 1] != colors[i] {
                dp1 = dp0.max(dp1) + nums[i] as i64;
            } else {
                dp1 = dp0 + nums[i] as i64;
            }
            dp0 = max;
        }
        dp0.max(dp1)
    }
}