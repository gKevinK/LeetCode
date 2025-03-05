impl Solution {
    pub fn count_complete_day_pairs(mut hours: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut v = vec![0; 24];
        for h in hours {
            v[(h % 24) as usize] += 1;
        }
        for i in 1..12 {
            r += v[i] * v[24 - i];
        }
        for c in [v[0], v[12]] {
            if c > 1 {
                r += c * (c - 1) / 2;
            }
        }
        r
    }
}