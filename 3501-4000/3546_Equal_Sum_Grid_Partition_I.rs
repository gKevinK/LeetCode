impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut sum_x = vec![0; m];
        let mut sum_y = vec![0; n];
        for x in 0..m {
            for y in 0..n {
                sum_x[x] += grid[x][y] as i64;
                sum_y[y] += grid[x][y] as i64;
            }
        }
        let sum: i64 = sum_x.iter().sum();
        let mut s = 0;
        for x in 0..m {
            s += sum_x[x];
            if s * 2 == sum {
                return true;
            }
        }
        s = 0;
        for y in 0..n {
            s += sum_y[y];
            if s * 2 == sum {
                return true;
            }
        }
        false
    }
}