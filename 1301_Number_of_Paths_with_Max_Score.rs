impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let MOD = 1_000_000_007;
        let n = board.len();
        let b: Vec<Vec<_>> = board.iter().map(|s| s.chars().collect()).collect();
        let mut dp = vec![vec![[0, 0]; n]; n];
        for i in 0..n {
            for j in 0..n {
                if b[i][j] == 'X' {
                    continue;
                }
                let mut s = 0;
                let mut p = 0;
                if i > 0 && dp[i - 1][j][1] > 0 {
                    if dp[i - 1][j][0] > s {
                        s = dp[i - 1][j][0];
                        p = dp[i - 1][j][1];
                    } else if dp[i - 1][j][0] == s {
                        p = (p + dp[i - 1][j][1]) % MOD;
                    }
                }
                if j > 0 && dp[i][j - 1][1] > 0 {
                    if dp[i][j - 1][0] > s {
                        s = dp[i][j - 1][0];
                        p = dp[i][j - 1][1];
                    } else if dp[i][j - 1][0] == s {
                        p = (p + dp[i][j - 1][1]) % MOD;
                    }
                }
                if i > 0 && j > 0 && dp[i - 1][j - 1][1] > 0 {
                    if dp[i - 1][j - 1][0] > s {
                        s = dp[i - 1][j - 1][0];
                        p = dp[i - 1][j - 1][1];
                    } else if dp[i - 1][j - 1][0] == s {
                        p = (p + dp[i - 1][j - 1][1]) % MOD;
                    }
                }
                dp[i][j][0] = s;
                if p > 0 && b[i][j] != 'S' {
                    dp[i][j][0] += b[i][j] as i32 - '0' as i32;
                }
                dp[i][j][1] = p;
                if b[i][j] == 'E' {
                    dp[i][j][1] += 1;
                }
            }
        }
        dp[n - 1][n - 1].to_vec()
    }
}