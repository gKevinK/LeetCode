impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let max = *grid.iter().map(|r| r.iter().max().unwrap()).max().unwrap() as usize;
        let mut tele = vec![i32::MAX; max + 2];
        let mut tele2 = vec![i32::MAX; max + 1];
        let mut dp = vec![i32::MAX / 2; n + 1];
        for ik in 0..=k as usize {
            tele2.fill(i32::MAX);
            dp.fill(i32::MAX / 2);
            dp[1] = -grid[0][0];
            for x in 0..m {
                for y in 0..n {
                    let num = grid[x][y];
                    dp[y + 1] = tele[num as usize].min(dp[y].min(dp[y + 1]) + num);
                    tele2[num as usize] = tele2[num as usize].min(dp[y + 1]);
                }
            }
            for i in (0..=max).rev() {
                tele[i] = tele[i + 1].min(tele2[i]);
            }
        }
        dp[n]
    }
}