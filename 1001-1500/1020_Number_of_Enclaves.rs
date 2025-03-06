impl Solution {
    pub fn num_enclaves(mut a: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let ds: Vec<(i32, i32)> = vec![ (1, 0), (0, 1), (-1, 0), (0, -1) ];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let m = a.len();
        let n = a[0].len();
        for i in 0..m {
            if a[i][0] == 1 {
                q.push_back((i, 0));
            }
            if a[i][n - 1] == 1 {
                q.push_back((i, n - 1));
            }
        }
        for j in 0..n {
            if a[0][j] == 1 {
                q.push_back((0, j));
            }
            if a[m - 1][j] == 1 {
                q.push_back((m - 1, j));
            }
        }
        while let Some(p) = q.pop_front() {
            if a[p.0][p.1] == 0 {
                continue;
            }
            a[p.0][p.1] = 0;
            for d in &ds {
                let x = (p.0 as i32 + d.0) as usize;
                let y = (p.1 as i32 + d.1) as usize;
                if x >= 0 && x < m && y >= 0 && y < n && a[x][y] == 1 {
                    q.push_back((x, y));
                }
            }
        }
        let mut c: i32 = 0;
        for r in a {
            for s in r {
                c += s;
            }
        }
        c
    }
}