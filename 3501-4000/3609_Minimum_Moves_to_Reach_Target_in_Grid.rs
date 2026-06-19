impl Solution {
    pub fn min_moves(sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, tx, ty));
        let mut set = std::collections::HashSet::new();
        while let Some((step, x, y)) = queue.pop_front() {
            if sx == x && sy == y {
                return step;
            }
            let mut cand = [(0, 0); 4];
            let mut n = 0;
            if x > y * 2 {
                if x % 2 == 0 {
                    cand[n] = (x / 2, y);
                    n += 1;
                }
            } else if y > x * 2 {
                if y % 2 == 0 {
                    cand[n] = (x, y / 2);
                    n += 1;
                }
            } else {
                if x >= y {
                    cand[n] = (x - y, y);
                    n += 1;
                }
                if x <= y {
                    cand[n] = (x, y - x);
                    n += 1;
                }
            }
            for i in 0..n {
                let (x2, y2) = cand[i];
                if x2 < sx || y2 < sy {
                    continue;
                }
                if !set.contains(&(x2, y2)) {
                    set.insert((x2, y2));
                    queue.push_back((step + 1, x2, y2));
                }
            }
        }
        -1
    }
}