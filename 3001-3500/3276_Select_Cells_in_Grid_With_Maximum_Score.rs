impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut values = Vec::with_capacity(m * n);
        for x in 0..m {
            for y in 0..n {
                values.push((grid[x][y], x));
            }
        }
        values.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut dp = vec![vec![0; 1 << m]; m * n];
        let all = 1 << m - 1;
        Self::f(&values, &mut dp, 0, 0)
    }

    fn f(values: &Vec<(i32, usize)>, mut dp: &mut Vec<Vec<i32>>, i: usize, mask: usize) -> i32 {
        let l = values.len();
        if i == l {
            return 0;
        }
        if dp[i][mask] != 0 {
            return dp[i][mask];
        }
        let mut res = Self::f(&values, &mut dp, i + 1, mask);
        if mask & (1 << values[i].1) == 0 {
            let mut j = i;
            while j < l && values[j].0 == values[i].0 {
                j += 1;
            }
            res = res.max(values[i].0 + Self::f(&values, &mut dp, j, mask | (1 << values[i].1)));
        }
        dp[i][mask] = res;
        res
    }
}