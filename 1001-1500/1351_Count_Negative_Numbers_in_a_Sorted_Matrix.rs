impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len() as i32;
        let mut i = 0;
        let mut j = n - 1;
        let mut r = 0;
        while i < m {
            while j >= 0 && grid[i][j as usize] < 0 {
                j -= 1;
            }
            r += n - j - 1;
            i += 1;
        }
        r
    }
}