impl Solution {
    pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        let MOD = 1_000_000_007;
        let mut dp = vec![vec![0; target as usize + 1]; 2];
        dp[0][0] = 1;
        for i in 0..d {
            for j in 0..=target {
                let mut s = 0;
                for k in std::cmp::max(0, j - f)..j {
                    s += dp[(i % 2) as usize][k as usize];
                    s %= MOD;
                }
                dp[((i + 1) % 2) as usize][j as usize] = s;
            }
        }
        dp[(d % 2) as usize][target as usize]
    }
}