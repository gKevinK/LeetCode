impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let MOD = 1_000_000_007;
        let n = s.len();
        let vs: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
        }
        for len in 1..n {
            for i in 0..(n - len) {
                let mut j = i + len;
                if vs[i] == vs[j] {
                    dp[i][j] = dp[i + 1][j - 1] * 2;
                    let mut left = i + 1;
                    let mut right = j - 1;
                    while left <= right && vs[left] != vs[i] {
                        left += 1;
                    }
                    while left <= right && vs[right] != vs[i] {
                        right -= 1;
                    }
                    if left > right {
                        dp[i][j] += 2;
                    } else if left == right {
                        dp[i][j] += 1;
                    } else {
                        dp[i][j] -= dp[left + 1][right - 1];
                    }
                } else {
                    dp[i][j] = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
                }
                if dp[i][j] < 0 {
                    dp[i][j] += MOD;
                } else {
                    dp[i][j] %= MOD;
                }
            }
        }
        dp[0][n - 1]
    }
}