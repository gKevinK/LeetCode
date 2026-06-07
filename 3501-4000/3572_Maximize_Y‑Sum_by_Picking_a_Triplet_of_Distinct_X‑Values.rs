impl Solution {
    pub fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
        let n = x.len();
        let mut s = std::collections::HashMap::new();
        for i in 0..n {
            let e = s.entry(x[i]).or_insert(0);
            *e = (*e).max(y[i]);
        }
        let mut m = [0, 0, 0];
        for &v in s.values() {
            if v > m[2] {
                if v > m[0] {
                    m = [v, m[0], m[1]];
                } else if v > m[1] {
                    m = [m[0], v, m[1]];
                } else {
                    m[2] = v;
                }
            }
        }
        if m[2] > 0 { m[0] + m[1] + m[2] } else { -1 }
    }
}