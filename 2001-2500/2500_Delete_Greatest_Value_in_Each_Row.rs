impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        let len = grid[0].len();
        for mut row in &mut grid {
            row.sort_unstable();
        }
        let mut res = 0;
        for j in 0..len {
            let mut max = 0;
            for i in 0..grid.len() {
                max = std::cmp::max(max, grid[i][j]);
            }
            res += max;
        }
        res
    }
}