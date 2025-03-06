impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut r = 0;
        let mut n = 0;
        for c in s.chars() {
            if c == 'E' {
                n += 1;
                r = std::cmp::max(r, n);
            } else {
                n = std::cmp::max(0, n - 1);
            }
        }
        r
    }
}