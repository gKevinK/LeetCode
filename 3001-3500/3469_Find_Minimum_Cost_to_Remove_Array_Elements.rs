impl Solution {
    pub fn min_cost(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        use std::cmp::min;
        let MAX = 1_000_000_001;
        let n = nums.len();
        let mut dp1 = vec![MAX; n + 1];
        let mut dp2 = vec![MAX; n + 1];
        dp2[0] = 0;
        let mut res = MAX;
        for i in (1..=n).step_by(2) {
            (dp1, dp2) = (dp2, dp1);
            for r in 0..n {
                if dp1[r] == MAX {
                    continue;
                }
                if i == n {
                    res = res.min(dp1[r] + nums[r]);
                    continue;
                }
                if i + 1 == n {
                    res = res.min(dp1[r] + max(nums[i], nums[r]));
                    continue;
                }
                dp2[r] = dp2[r].min(dp1[r] + max(nums[i], nums[i + 1]));
                dp2[i] = dp2[i].min(dp1[r] + max(nums[r], nums[i + 1]));
                dp2[i + 1] = dp2[i + 1].min(dp1[r] + max(nums[r], nums[i]));
                dp1[r] = MAX;
            }
        }
        res
    }
}