impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let vd = [ [0, 1], [1, 0], [0, -1], [-1, 0] ];
        let vs = [ [0, 2], [1, 3], [1, 2], [0, 1], [2, 3], [0, 3] ];
        let mut v = Vec::new();
        for &d in &vs[grid[0][0] as usize - 1] {
            if d == 0 || d == 1 {
                v.push(d);
            }
        }
        for mut d in v {
            let mut x = 0;
            let mut y = 0;
            loop {
                if x == m - 1 && y == n - 1 {
                    return true;
                }
                let x2 = x + vd[d][0];
                let y2 = y + vd[d][1];
                if x2 < 0 || x2 >= m || y2 < 0 || y2 >= n {
                    break;
                }
                d = (d + 2) % 4;
                let cd = vs[grid[x2 as usize][y2 as usize] as usize - 1];
                if cd[0] != d && cd[1] != d {
                    break;
                }
                x = x2;
                y = y2;
                d = if cd[0] == d { cd[1] } else { cd[0] };
                if x == 0 && y == 0 {
                    break;
                }
            }
        }
        false
    }
}