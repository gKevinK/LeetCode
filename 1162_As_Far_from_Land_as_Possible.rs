impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let N = grid.len();
        let Ni = N as i32;
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut visit = vec![vec![false; N]; N];
        let mut round = 0;
        let dir = [ -1, 0, 1, 0, -1 ];
        for x in 0..N {
            for y in 0..N {
                if grid[x][y] == 1 {
                    for d in 0..4 {
                        let i = x as i32 + dir[d];
                        let j = y as i32 + dir[d + 1];
                        if i < 0 || j < 0 || i >= Ni || j >= Ni {
                            continue;
                        }
                        let i1 = i as usize;
                        let j1 = j as usize;
                        if grid[i1][j1] == 0 && visit[i1][j1] == false {
                            q.push_back((i, j));
                            visit[i1][j1] = true;
                        }
                    }
                }
            }
        }
        if q.is_empty() {
            return -1;
        }
        while !q.is_empty() {
            round += 1;
            let l = q.len();
            for _ in 0..l {
                let f = q.pop_front().unwrap();
                for d in 0..4 {
                    let i = f.0 + dir[d];
                    let j = f.1 + dir[d + 1];
                    if i < 0 || j < 0 || i >= Ni || j >= Ni {
                        continue;
                    }
                    let i1 = i as usize;
                    let j1 = j as usize;
                    if grid[i1][j1] == 0 && visit[i1][j1] == false {
                        q.push_back((i, j));
                        visit[i1][j1] = true;
                    }
                }
            }
        }
        round
    }
}