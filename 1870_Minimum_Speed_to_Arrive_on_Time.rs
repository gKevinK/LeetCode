impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let mut lo = 1;
        let mut hi = std::i32::MAX;
        if hour + 1.0 < dist.len() as f64 {
            return -1;
        }
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            let mut time = 0;
            for &d in &dist {
                time += (d + mi - 1) / mi;
            }
            let d = *dist.last().unwrap();
            if d % mi != 0 && hour < (time as f64) && (time as f64) < hour + 1.0 {
                if ((d % mi) as f64) <= mi as f64 * hour.fract() + 1e-8 {
                    time -= 1;
                }
            }
            if time as f64 > hour {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        lo
    }
}