impl Solution {
    pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: Vec<Vec<i32>>) -> bool {
        let n = circles.len();
        let xc = x_corner as i64;
        let yc = y_corner as i64;
        let mut set = (0..n + 2).map(|i| (i, 0)).collect::<Vec<_>>();

        let find = |mut set: &mut Vec<(usize, usize)>, i: usize| {
            let mut r = set[i].0;
            if set[r].0 != r {
                r = set[r].0;
            }
            set[i].0 = r;
            r
        };

        let unite = |mut set: &mut Vec<(usize, usize)>, i: usize, j: usize| {
            let mut ri = find(&mut set, i);
            let mut rj = find(&mut set, j);
            if ri == rj {
                return;
            }
            if set[ri].1 < set[rj].1 {
                (ri, rj) = (rj, ri);
            }
            set[rj].0 = ri;
            set[ri].1 = set[ri].1.max(set[rj].1 + 1);
        };

        let mut skip = vec![false; n];
        for i in 0..n {
            let x1 = circles[i][0] as i64;
            let y1 = circles[i][1] as i64;
            let r1 = circles[i][2] as i64;
            if x1 < -r1 || x1 > xc + r1 || y1 < -r1 || y1 > yc + r1 {
                skip[i] = true;
            }
        }
        for i in 0..n {
            if skip[i] {
                continue;
            }
            let x1 = circles[i][0] as i64;
            let y1 = circles[i][1] as i64;
            let r1 = circles[i][2] as i64;
            for j in (i + 1)..n {
                if skip[j] {
                    continue;
                }
                let x2 = circles[j][0] as i64;
                let y2 = circles[j][1] as i64;
                let r2 = circles[j][2] as i64;
                if (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) <= (r1 + r2) * (r1 + r2) {
                    if (x1 > xc && y2 > yc) || (x2 > xc && y1 > yc) {
                        let (dx1, dx2) = (x1.max(x2) - xc, x1.min(x2) - xc);
                        let (dy1, dy2) = (y1.min(y2) - yc, y1.max(y2) - yc);
                        if dx1 * dy2 > dx2 * dy1 {
                            continue;
                        }
                    }
                    unite(&mut set, i, j);
                }
            }
            if x1 * x1 + y1 * y1 <= r1 * r1 || (xc - x1) * (xc - x1) + (yc - y1) * (yc - y1) <= r1 * r1 {
                return false;
            }
            if find(&mut set, i) != find(&mut set, n) {
                if (0 <= x1 && x1 <= xc && yc - r1 <= y1 && y1 <= yc + r1)
                    || (0 <= y1 && y1 <= yc && -r1 <= x1 && x1 <= r1)
                    || x1 * x1 + (yc - y1) * (yc - y1) <= r1 * r1 {
                        unite(&mut set, i, n);
                    }
            }
            if find(&mut set, i) != find(&mut set, n + 1) {
                if (0 <= x1 && x1 <= xc && -r1 <= y1 && y1 <= r1)
                    || (0 <= y1 && y1 <= yc && xc - r1 <= x1 && x1 <= xc + r1)
                    || (xc - x1) * (xc - x1) + y1 * y1 <= r1 * r1 {
                        unite(&mut set, i, n + 1);
                    }
            }
            if find(&mut set, n) == find(&mut set, n + 1) {
                return false;
            }
        }
        find(&mut set, n) != find(&mut set, n + 1)
    }
}