impl Solution {
    pub fn max_total(value: Vec<i32>, limit: Vec<i32>) -> i64 {
        let n = value.len();
        let mut v = Vec::from_iter((0..n).map(|i| (limit[i], -value[i])));
        v.sort_unstable();
        let mut i = 0;
        let mut res = 0;
        while i < n {
            let lim = v[i].0;
            let mut j = i;
            while j < n && v[j].0 == lim && j - i < lim as usize {
                res -= v[j].1 as i64;
                j += 1;
            }
            while j < n && v[j].0 == lim {
                j += 1;
            }
            i = j;
        }
        res
    }
}