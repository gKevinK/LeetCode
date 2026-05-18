impl Solution {
    pub fn min_moves(matrix: Vec<String>) -> i32 {
        let mut grid = matrix.iter().map(|s| s.as_bytes().to_vec()).collect::<Vec<_>>();
        let m = grid.len();
        let n = grid[0].len();
        if grid[m - 1][n - 1] == b'#' {
            return -1;
        }
        let mut tele = vec![vec![]; 26];
        for x in 0..m {
            for y in 0..n {
                if b'A' <= grid[x][y] && grid[x][y] <= b'Z' {
                    tele[(grid[x][y] - b'A') as usize].push((x, y));
                }
            }
        }
        let mut dp = vec![vec![false; n]; m];
        let mut q = std::collections::VecDeque::new();

        if grid[0][0] == b'.' {
            dp[0][0] = true;
            q.push_back((0, 0, 0));
        } else {
            let ord = (grid[0][0] - b'A') as i32;
            for &(x2, y2) in &tele[ord as usize] {
                dp[x2][y2] = true;
                q.push_back((0, x2, y2));
            }
        }
        if dp[m - 1][n - 1] {
            return 0;
        }
        while let Some((moves, x, y)) = q.pop_front() {
            for (x2, y2) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if x2 >= m || y2 >= n || grid[x2][y2] == b'#' || dp[x2][y2] {
                    continue;
                }
                if grid[x2][y2] == b'.' {
                    dp[x2][y2] = true;
                    q.push_back((moves + 1, x2, y2));
                } else {
                    let ord = (grid[x2][y2] - b'A') as i32;
                    for &(x3, y3) in &tele[ord as usize] {
                        dp[x3][y3] = true;
                        q.push_back((moves + 1, x3, y3));
                    }
                }
                if dp[m - 1][n - 1] {
                    return moves + 1;
                }
            }
        }
        -1
    }
}