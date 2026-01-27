impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, cost: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut nsum = vec![0; n + 1];
        let mut csum = vec![0; n + 1];
        for i in 0..n {
            nsum[i + 1] = nsum[i] + nums[i] as i64;
            csum[i + 1] = csum[i] + cost[i] as i64;
        }
        let mut ctotal = csum[n];

        let mut dp = vec![i64::MAX; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            for j in 0..i {
                let cs = csum[i] - csum[j];
                let c = nsum[i] * cs + k as i64 * (ctotal - csum[j]);
                dp[i] = dp[i].min(dp[j] + c);
            }
        }
        dp[n]
    }
}