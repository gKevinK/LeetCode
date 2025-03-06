impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp1 = vec![vec![-1; n]; n];
        let mut dp2 = vec![vec![-1; n]; n];
        dp1[0][n - 1] = grid[0][0] + grid[0][n - 1];
        for r in 1..m {
            for i in 0..n {
                for j in 0..n {
                    let mut t = -1;
                    for x in 0..=2 {
                        for y in 0..=2 {
                            if (i == 0 && x == 0) || (j == 0 && y == 0) || i + x - 1 >= n || j + y - 1 >= n {
                                continue;
                            }
                            t = std::cmp::max(t, dp1[i + x - 1][j + y - 1]);
                        }
                    }
                    if t < 0 {
                        continue;
                    }
                    dp2[i][j] = t + if i == j { grid[r][i] } else { grid[r][i] + grid[r][j] };
                }
            }
            let tmp = dp1;
            dp1 = dp2;
            dp2 = tmp;
        }
        dp1.iter().map(|v| *v.iter().max().unwrap()).max().unwrap()
    }
}