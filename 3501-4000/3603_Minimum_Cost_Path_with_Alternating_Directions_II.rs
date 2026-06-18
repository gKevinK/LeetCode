impl Solution {
    pub fn min_cost(m: i32, n: i32, wait_cost: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let MAX = i64::MAX / 2;
        let mut dp1 = vec![MAX; n];
        let mut dp2 = vec![MAX; n];
        dp2[0] = 1;
        for x in 0..m {
            for y in 0..n {
                let cost = ((x + 1) * (y + 1)) as i64 + wait_cost[x][y] as i64;
                if x > 0 {
                    if y > 0 {
                        dp2[y] = dp1[y].min(dp2[y - 1]) + cost;
                    } else {
                        dp2[y] = dp1[y] + cost;
                    }
                } else if y > 0 {
                    dp2[y] = dp2[y - 1] + cost;
                }
            }
            std::mem::swap(&mut dp1, &mut dp2);
        }
        dp1[n - 1] - wait_cost[m - 1][n - 1] as i64
    }
}