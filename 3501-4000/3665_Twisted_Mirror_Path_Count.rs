impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let m = grid.len();
        let n = grid[0].len();
        let mut r = vec![0; n];
        let mut d = vec![0; n];
        r[0] = 1;
        d[0] = 1;
        for x in 0..m {
            for y in 0..n {
                let from_u = d[y];
                let from_l = if y > 0 { r[y - 1] } else { 0 };
                if grid[x][y] == 1 {
                    r[y] = from_u;
                    d[y] = from_l;
                } else {
                    let mut total = from_u + from_l;
                    total %= MOD;
                    r[y] = total;
                    d[y] = total;
                }
            }
        }
        r[n - 1]
    }
}