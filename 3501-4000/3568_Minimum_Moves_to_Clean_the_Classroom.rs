impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let mut grid = classroom.iter().map(|s| s.as_bytes().to_vec()).collect::<Vec<_>>();
        let m = grid.len();
        let n = grid[0].len();
        let mut queue = std::collections::VecDeque::new();
        let mut lnum = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == b'L' {
                    grid[i][j] = b'0' + lnum;
                    lnum += 1;
                } else if grid[i][j] == b'S' {
                    queue.push_back((i, j, 0, energy, 0));
                }
            }
        }
        if lnum == 0 {
            return 0;
        }
        let lmask = (1 << lnum as usize) - 1;
        let idx = |x, y, mask| (x * n + y) * (lmask + 1) + mask;
        let mut dp = vec![-100; m * n * (lmask + 1)];
        dp[idx(queue[0].0, queue[0].1, 0)] = 0;
        while let Some((x0, y0, mask0, e0, step0)) = queue.pop_front() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x, y) = (x0 + dx as usize, y0 + dy as usize);
                if x >= m || y >= n || grid[x][y] == b'X' {
                    continue;
                }
                let mut e = e0 - 1;
                let mut mask = mask0;
                if b'0' <= grid[x][y] && grid[x][y] <= b'9' {
                    let lid = (grid[x][y] - b'0') as usize;
                    mask |= 1 << lid;
                    if mask == lmask {
                        return step0 + 1;
                    }
                } else if grid[x][y] == b'R' {
                    e = energy;
                }
                let old_e = dp[idx(x, y, mask)];
                if e > 0 && e > old_e {
                    dp[idx(x, y, mask)] = old_e.max(e);
                    queue.push_back((x, y, mask, e, step0 + 1));
                }
            }
        }
        -1
    }
}