impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let mut min: i32 = 257;
        let mut max: i32 = -1;
        let mut median1: i32 = 0;
        let mut median2: i32 = 0;
        let mut sum: f64 = 0.0;
        let mut scount: i32 = 0;
        let mut mc: i32 = 0;
        let mut mode: i32 = -1;
        for (i, &item) in count.iter().enumerate() {
            if item > 0 {
                min = std::cmp::min(min, i as i32);
                max = std::cmp::max(max, i as i32);
                sum += i as f64 * item as f64;
                scount += item as i32;
                if item > mc {
                    mc = item;
                    mode = i as i32;
                }
            }
        }
        let mut c = scount;
        for (i, &item) in count.iter().enumerate() {
            if item > 0 {
                if c > 0 {
                    c -= item * 2;
                    if c <= 0 {
                        median1 = i as i32;
                        median2 = i as i32;
                    }
                } else if c == 0 {
                    median2 = i as i32;
                    c = -1;
                }
            }
        }
        vec![min as f64, max as f64, sum / scount as f64, (median1 + median2) as f64 / 2 as f64, mode as f64]
    }
}