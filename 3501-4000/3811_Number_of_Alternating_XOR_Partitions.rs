impl Solution {
    pub fn alternating_xor(nums: Vec<i32>, target1: i32, target2: i32) -> i32 {
        let MOD = 1_000_000_007;
        let n = nums.len();
        let v0 = 0;
        let v1 = target1;
        let v2 = target1 ^ target2;
        let v3 = target2;

        let mut dp = [0; 4];
        dp[0] = 1;
        let mut p = 0;
        for i in 0..n - 1 {
            p ^= nums[i];
            let mut new = [0; 4];
            if p == v1 {
                new[1] = dp[0];
            }
            if p == v2 {
                new[2] = dp[1];
            }
            if p == v3 {
                new[3] = dp[2];
            }
            if p == v0 {
                new[0] = dp[3];
            }
            for k in 0..4 {
                dp[k] = (dp[k] + new[k]) % MOD;
            }
        }
        p ^= nums[n - 1];
        let mut res = 0;
        if p == v1 {
            res = (res + dp[0]) % MOD;
        }
        if p == v2 {
            res = (res + dp[1]) % MOD;
        }
        if p == v3 {
            res = (res + dp[2]) % MOD;
        }
        if p == v0 {
            res = (res + dp[3]) % MOD;
        }
        res
    }
}