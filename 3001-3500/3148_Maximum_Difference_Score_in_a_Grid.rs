impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        const MIN: i32 = -2_000_000_001;
        let mut dp = vec![MIN; n];
        let mut res = MIN;
        for x in 0..m {
            let mut lmax = MIN;
            for y in 0..n {
                res = res.max(grid[x][y] + lmax);
                res = res.max(grid[x][y] + dp[y]);
                lmax = lmax.max(-grid[x][y]);
                dp[y] = dp[y].max(lmax);
            }
        }
        res
    }
}