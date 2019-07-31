impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, r0: i32, c0: i32, color: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let oc = grid[r0 as usize][c0 as usize];
        let mut res = vec![vec![0; n]; m];
        let mut q = std::collections::VecDeque::new();
        q.push_back((r0 as usize, c0 as usize));
        let v = vec![-1, 0, 1, 0, -1];
        while let Some(a) = q.pop_front() {
            let mut border = false;
            for i in 0..4 {
                let _x = a.0 as i32 + v[i];
                let _y = a.1 as i32 + v[i + 1];
                if _x < 0 || _x >= m as i32 || _y < 0 || _y >= n as i32 {
                    border = true;
                    continue;
                }
                let x = _x as usize;
                let y = _y as usize;
                if grid[x][y] != oc {
                    border = true;
                } else if res[x][y] == 0 {
                    q.push_back((x, y));
                    res[x][y] = oc;
                }
            }
            if border {
                res[a.0][a.1] = color;
            }
        }
        for i in 0..m {
            for j in 0..n {
                if res[i][j] == 0 {
                    res[i][j] = grid[i][j];
                }
            }
        }
        res
    }
}