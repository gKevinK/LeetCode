impl Solution {
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        let mut x = 50f64;
        let mut y = 50f64;
        let mut r = 10f64;
        let d = 0.000001f64;
        let f = |xc: f64, yc: f64| {
            let mut r = 0f64;
            for p in &positions {
                r += ((p[0] as f64 - xc).powi(2) + (p[1] as f64 - yc).powi(2)).sqrt();
            }
            r
        };
        let mut lx = 0f64;
        let mut ly = 0f64;
        loop {
            let s = f(x, y);
            let sx = f(x + d, y);
            let sy = f(x, y + d);
            if lx * (s - sx) < 0.0 || ly * (s - sy) < 0.0 {
                r *= 0.2;
            }
            lx = s - sx;
            ly = s - sy;
            let l = lx.abs() + ly.abs();
            if l / d < 0.000001 || r < 0.000001 {
                return f(x, y);
            }
            x += r * lx / l;
            y += r * ly / l;
        }
    }
}