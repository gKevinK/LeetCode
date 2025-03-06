impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        if n < d as usize {
            return -1;
        }
        let mut dp = vec![vec![1_000_000; n + 1]; d as usize + 1];
        dp[0][0] = 0;
        for i in 0..d as usize {
            for j in i..n {
                let mut m = 0;
                for k in j..n {
                    m = std::cmp::max(m, job_difficulty[k]);
                    dp[i + 1][k + 1] = std::cmp::min(dp[i + 1][k + 1],
                        dp[i][j] + m);
                }
            }
        }
        dp[d as usize][n]
    }
}