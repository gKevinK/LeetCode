impl Solution {
    pub fn last_integer(n: i64) -> i64 {
        let mut start = 1;
        let mut step = 1;
        let mut end = n;
        let mut i = 0;
        while start != end {
            i += 1;
            if i % 2 == 0 && (end - start) / step % 2 == 1 {
                start += step;
            }
            step *= 2;
            end = start + (end - start) / step * step;
        }
        start
    }
}