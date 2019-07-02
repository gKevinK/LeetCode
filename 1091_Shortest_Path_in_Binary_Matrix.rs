impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let n = grid.len() as i32;
        let mut visit = vec![vec![false; grid.len()]; grid.len()];
        let mut deque: VecDeque<(i32, i32, i32)> = VecDeque::new();
        if grid[0][0] == 1 || grid[(n - 1) as usize][(n - 1) as usize] == 1 {
            return -1;
        }
        deque.push_back((0, 0, 1));
        visit[0][0] = true;
        while let Some((i, j, l)) = deque.pop_front() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let x = i + dx;
                    let y = j + dy;
                    if x < 0 || x >= n || y < 0 || y >= n || visit[x as usize][y as usize] || grid[x as usize][y as usize] == 1 {
                        continue;
                    }
                    if x + 1 == n && y + 1 == n {
                        return l + 1;
                    }
                    deque.push_back((x, y, l + 1));
                    visit[x as usize][y as usize] = true;
                }
            }
        }
        -1
    }
}