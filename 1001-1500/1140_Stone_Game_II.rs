impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut dp = vec![vec![0; n + 2]; n];
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + piles[i];
        }
        for i in (0..n).rev() {
            for m in 1..=(i + 1) {
                let mut t = 0;
                for j in 1..=2 * m {
                    if i + j >= n {
                        t = std::cmp::max(t, sum[i + j] - sum[i]);
                        break;
                    }
                    t = std::cmp::max(t, sum[n] - sum[i] - dp[i + j][std::cmp::max(m, j)]);
                }
                dp[i][m] = t;
            }
        }
        dp[0][1]
    }
}