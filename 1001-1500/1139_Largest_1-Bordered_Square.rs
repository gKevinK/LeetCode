impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![(0, 0); n + 1]; m + 1];
        let mut r = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    dp[i + 1][j + 1] = (dp[i][j + 1].0 + 1, dp[i + 1][j].1 + 1);
                }
                for l in (r + 1)..=std::cmp::min(dp[i + 1][j + 1].0, dp[i + 1][j + 1].1) {
                    if dp[i + 2 - l][j + 1].1 >= l && dp[i + 1][j + 2 - l].0 >= l {
                        r = std::cmp::max(r, l);
                    }
                }
            }
        }
        (r as i32).pow(2)
    }
}