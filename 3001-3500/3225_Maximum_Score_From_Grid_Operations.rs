impl Solution {
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let m = grid.len();
        let n = grid[0].len();
        let mut last0 = vec![0; m + 1];
        let mut last1 = vec![0; m + 1];
        let mut curr0 = vec![0; m + 1];
        let mut curr1 = vec![0; m + 1];
        for y in 1..n {
            std::mem::swap(&mut curr0, &mut last0);
            std::mem::swap(&mut curr1, &mut last1);
            for x in 0..=m {
                let mut v0 = last1[x];
                let mut v1 = last1[x];
                let mut add = 0;
                for x2 in (0..x).rev() {
                    add += grid[x2][y - 1] as i64;
                    v0 = v0
                        .max(last0[x2] + add)
                        .max(last1[x2]);
                    v1 = v1
                        .max(last0[x2] + add)
                        .max(last1[x2]);
                }
                add = 0;
                for x2 in (x + 1)..=m {
                    add += grid[x2 - 1][y] as i64;
                    v0 = v0
                        .max(last0[x2])
                        .max(last1[x2]);
                    v1 = v1
                        .max(last0[x2] + add)
                        .max(last1[x2] + add);
                }
                curr0[x] = v0;
                curr1[x] = v1;
            }
        }
        *curr1.iter().max().unwrap()
    }
}