impl Solution {
    pub fn max_rectangle_area(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut xs = HashMap::new();
        let mut ys = HashMap::new();
        for p in &points {
            *xs.entry(p[0]).or_insert(0) += 1;
            *ys.entry(p[1]).or_insert(0) += 1;
        }
        let vx = xs.keys().cloned().collect::<Vec<_>>();
        let vy = ys.keys().cloned().collect::<Vec<_>>();
        let mut res = -1;
        for x1 in &vx {
            for x2 in &vx {
                if x1 >= x2 {
                    continue;
                }
                for y1 in &vy {
                    for y2 in &vy {
                        if y1 >= y2 {
                            continue;
                        }
                        let mut f = [false; 5];
                        f[4] = true;
                        for p in &points {
                            if p[0] == *x1 && p[1] == *y1 {
                                f[0] = true;
                            } else if p[0] == *x2 && p[1] == *y1 {
                                f[1] = true;
                            } else if p[0] == *x1 && p[1] == *y2 {
                                f[2] = true;
                            } else if p[0] == *x2 && p[1] == *y2 {
                                f[3] = true;
                            } else if *x1 <= p[0] && p[0] <= *x2 && *y1 <= p[1] && p[1] <= *y2 {
                                f[4] = false;
                                break;
                            }
                        }
                        if f[0] && f[1] && f[2] && f[3] && f[4] {
                            res = res.max((x2 - x1) * (y2 - y1));
                        }
                    }
                }
            }
        }
        res
    }
}