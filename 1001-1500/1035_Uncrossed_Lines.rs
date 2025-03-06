impl Solution {
    pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; b.len() + 1]; a.len() + 1];
        for i in 0..a.len() {
            for j in 0..b.len() {
                dp[i + 1][j + 1] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
                if a[i] == b[j] {
                    dp[i + 1][j + 1] = std::cmp::max(dp[i + 1][j + 1], dp[i][j] + 1);
                }
            }
        }
        dp[a.len()][b.len()]
    }
}