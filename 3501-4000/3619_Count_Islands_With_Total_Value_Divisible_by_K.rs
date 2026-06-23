impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut visit = vec![false; m * n];
        let mut queue = std::collections::VecDeque::new();
        let mut res = 0;
        for x in 0..m {
            for y in 0..n {
                if !visit[x * n + y] && grid[x][y] > 0 {
                    let mut sum = grid[x][y] as i64;
                    visit[x * n + y] = true;
                    queue.push_back((x, y));
                    while let Some((i, j)) = queue.pop_front() {
                        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let i2 = (i as i32 + dx) as usize;
                            let j2 = (j as i32 + dy) as usize;
                            if i2 >= m || j2 >= n {
                                continue;
                            }
                            if !visit[i2 * n + j2] && grid[i2][j2] > 0 {
                                visit[i2 * n + j2] = true;
                                sum += grid[i2][j2] as i64;
                                queue.push_back((i2, j2));
                            }
                        }
                    }
                    if sum % (k as i64) == 0 {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}