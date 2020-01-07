impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        use std::cmp::min;

        let v: Vec<_> = s.chars().collect();
        let n = v.len();
        let mut dp = vec![vec![0; n + 1]; n];
        for len in 1..=n {
            for i in 0..=(n - len) {
                let j = i + len;
                if len == 1 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = min(dp[i][j - 1] + 1, dp[i + 1][j] + 1);
                    if v[i] == v[j - 1] {
                        dp[i][j] = min(dp[i][j], dp[i + 1][j - 1]);
                    }
                }
            }
        }
        dp[0][n]
    }
}