impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, mut walls: Vec<i32>) -> i32 {
        walls.sort_unstable();
        let n_robots = robots.len();
        let mut robots2 = (0..n_robots).map(|i| (robots[i], distance[i])).collect::<Vec<_>>();
        robots2.sort_unstable();
        // println!("{:?}", walls);
        // println!("{:?}", robots2);
        let mut sl = (0, 0);
        let mut sr = (0, 0);
        let mut wi = 0;
        for i in 0..n_robots {
            let (r, d) = robots2[i];
            let next = if i + 1 < n_robots { robots2[i + 1].0 } else { 1_000_000_001 };
            let mut wj = wi;
            while wj < walls.len() && walls[wj] <= r {
                wj += 1;
            }
            let mut count = 0;
            let mut wk = wj;
            while wk > wi && walls[wk - 1] >= r - d {
                wk -= 1;
            }
            let mut sl2 = (sl.0 + (wj - wk) as i32, r);
            wk = wj;
            while wk > wi && walls[wk - 1] >= r - d && walls[wk - 1] > sr.1 {
                wk -= 1;
            }
            if sr.0 + (wj - wk) as i32 > sl2.0 {
                sl2 = (sr.0 + (wj - wk) as i32, r);
            }
            wk = wj;
            let mut rwall = r;
            while wk < walls.len() && walls[wk] < next && walls[wk] <= r + d {
                rwall = walls[wk];
                wk += 1;
            }
            let sr2 = (sl.0.max(sr.0) + (wk - wj) as i32 + i32::from(wj > 0 && walls[wj - 1] == r), rwall);
            sl = sl2;
            sr = sr2;
            wi = wj;
            // println!("{} {:?} {:?}", i, sl, sr);
        }
        sl.0.max(sr.0)
    }
}