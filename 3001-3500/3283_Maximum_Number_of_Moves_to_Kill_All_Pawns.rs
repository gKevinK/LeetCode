impl Solution {
    pub fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
        let idx = |x: usize, y: usize| { x * 50 + y };
        let idxd = |mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize| {
            if x1 >= 25 {
                (x1, x2) = (49 - x1, 49 - x2);
            }
            if y1 >= 25 {
                (y1, y2) = (49 - y1, 49 - y2);
            }
            (x1 * 25 + y1) * 2500 + idx(x2, y2)
        };
        let MAX = 1 << 20;
        use std::sync::OnceLock;
        static MOVES: OnceLock<Vec<usize>> = OnceLock::new();
        let moves = MOVES.get_or_init(|| {
            let mut moves = vec![MAX; 25 * 25 * 50 * 50];
            let mut queue = std::collections::VecDeque::new();
            for x1 in 0..25 {
                for y1 in 0..25 {
                    moves[idxd(x1, y1, x1, y1)] = 0;
                    queue.push_back((x1, y1, 0));
                    while let Some((x, y, m)) = queue.pop_front() {
                        for (dx, dy) in [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)] {
                            let x2 = (x as i32 + dx) as usize;
                            let y2 = (y as i32 + dy) as usize;
                            if x2 >= 50 || y2 >= 50 {
                                continue;
                            }
                            let i = idxd(x1, y1, x2, y2);
                            if moves[i] == MAX {
                                moves[i] = m + 1;
                                queue.push_back((x2, y2, m + 1));
                            }
                        }
                    }
                }
            }
            moves
        });

        let n = positions.len();
        let all = (1 << n) - 1;
        let mut dp = vec![vec![0; all + 1]; n];
        for round in 2..=n {
            let alice = (round + n) % 2 == 1;
            for i in 0..n {
                let x1 = positions[i][0] as usize;
                let y1 = positions[i][1] as usize;
                for m in 0..=all {
                    if m.count_ones() != round as u32 || (m & (1 << i) == 0) {
                        continue;
                    }
                    let mut curr = if alice { 0 } else { MAX };
                    let m2 = m - (1 << i);
                    for j in 0..n {
                        if i == j {
                            continue;
                        }
                        if m2 & (1 << j) > 0 {
                            let x2 = positions[j][0] as usize;
                            let y2 = positions[j][1] as usize;
                            let v = dp[j][m2] + moves[idxd(x1, y1, x2, y2)];
                            curr = if alice { curr.max(v) } else { curr.min(v) };
                        }
                    }
                    dp[i][m] = curr;
                }
            }
        }
        (0..n).map(|i| {
            moves[idxd(kx as usize, ky as usize, positions[i][0] as usize, positions[i][1] as usize)] + dp[i][all]
        }).max().unwrap() as i32
    }
}