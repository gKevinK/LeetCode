impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut g = grid.clone();
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;
        for i in 0..(k / 2) {
            for j in 0..k {
                let t = g[x + i][y + j];
                g[x + i][y + j] = g[x + k - i - 1][y + j];
                g[x + k - i - 1][y + j] = t;
            }
        }
        g
    }
}