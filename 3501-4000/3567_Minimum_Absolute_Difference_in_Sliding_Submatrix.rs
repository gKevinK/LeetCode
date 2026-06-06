impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let mut res = vec![vec![0; n - k + 1]; m - k + 1];
        let mut val = Vec::with_capacity(k * k);
        for i in 0..res.len() {
            for j in 0..res[0].len() {
                val.clear();
                for x in 0..k {
                    val.extend_from_slice(&grid[i + x][j..j + k]);
                }
                val.sort_unstable();
                let mut diff = (val[k * k - 1] - val[0]).abs();
                if diff > 0 {
                    for x in 1..(k * k) {
                        if val[x] != val[x - 1] {
                            diff = diff.min((val[x] - val[x - 1]).abs());
                        }
                    }
                }
                res[i][j] = diff;
            }
        }
        res
    }
}