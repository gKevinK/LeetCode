impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut dp = vec![vec![(0, 0); len + 1]; len + 1];
        for i in 0..len {
            dp[i][i + 1] = (0, arr[i]);
        }
        for l in 2..=len {
            for i in 0..(len - l + 1) {
                let j = i + l;
                let mut r = std::i32::MAX;
                for k in (i + 1)..j {
                    r = std::cmp::min(r, dp[i][k].0 + dp[k][j].0 + dp[i][k].1 * dp[k][j].1);
                }
                dp[i][j] = (r, std::cmp::max(dp[i][i + 1].1, dp[i + 1][j].1));
            }
        }
        dp[0][len].0
    }
}