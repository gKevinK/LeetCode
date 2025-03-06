impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut m0 = 0;
        let mut m1 = 0;
        let mut c0 = 0;
        let mut c1 = 0;
        for ch in s.chars() {
            if ch == '0' {
                c0 += 1;
                m0 = std::cmp::max(m0, c0);
                c1 = 0;
            } else {
                c1 += 1;
                m1 = std::cmp::max(m1, c1);
                c0 = 0;
            }
        }
        m0 < m1
    }
}