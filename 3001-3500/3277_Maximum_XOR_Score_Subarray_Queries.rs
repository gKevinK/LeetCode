impl Solution {
    pub fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = nums[i];
        }
        for len in 2..=n {
            for i in 0..=(n - len) {
                let j = i + len - 1;
                dp[i][j] = dp[i][j - 1] ^ dp[i + 1][j];
            }
        }
        for len in 2..=n {
            for i in 0..=(n - len) {
                let j = i + len - 1;
                dp[i][j] = dp[i][j].max(dp[i + 1][j].max(dp[i][j - 1]));
            }
        }

        let mut res = Vec::with_capacity(queries.len());
        for q in &queries {
            res.push(dp[q[0] as usize][q[1] as usize]);
        }
        res
    }
}