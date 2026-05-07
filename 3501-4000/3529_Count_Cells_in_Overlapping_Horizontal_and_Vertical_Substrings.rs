impl Solution {
    pub fn count_cells(grid: Vec<Vec<char>>, pattern: String) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let p: Vec<_> = pattern.chars().collect();
        let mut g = vec![vec![0u8; n]; m];
        let mut kmp = vec![0; p.len()];
        for i in 1..p.len() {
            let mut j = kmp[i - 1];
            while j > 0 && p[i] != p[j] {
                j = kmp[j - 1];
            }
            if p[i] == p[j] {
                j += 1;
            }
            kmp[i] = j;
        }

        let mut ln = 0;
        let mut len = 0;
        for i in 0..m {
            for j in 0..n {
                while len > 0 && grid[i][j] != p[len] {
                    len = kmp[len - 1];
                }
                if grid[i][j] == p[len] {
                    len += 1;
                    if len == p.len() {
                        let mut sn = (i * n + j + 1 - p.len()).max(ln);
                        while sn < i * n + j + 1 {
                            let (si, sj) = (sn / n, sn % n);
                            g[si][sj] |= 1;
                            sn += 1;
                        }
                        ln = sn;
                        len = kmp[len - 1];
                    }
                }
            }
        }
        (ln, len) = (0, 0);
        for j in 0..n {
            for i in 0..m {
                while len > 0 && grid[i][j] != p[len] {
                    len = kmp[len - 1];
                }
                if grid[i][j] == p[len] {
                    len += 1;
                    if len == p.len() {
                        let mut sn = (j * m + i + 1 - p.len()).max(ln);
                        while sn < j * m + i + 1 {
                            let (sj, si) = (sn / m, sn % m);
                            g[si][sj] |= 2;
                            sn += 1;
                        }
                        ln = sn;
                        len = kmp[len - 1];
                    }
                }
            }
        }

        g.iter().map(|r| r.iter().map(|c| if *c == 3 { 1 } else { 0 }).sum::<i32>()).sum()
    }
}