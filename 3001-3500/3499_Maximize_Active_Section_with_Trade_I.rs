impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut last1 = 0;
        let mut last0 = 0;
        let mut c1 = 0;
        let mut min1 = 1000000;
        let mut max0 = 0;
        let mut max = 0;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && s[i] == s[j] {
                j += 1;
            }
            let len = (j - i) as i32;
            if s[i] == b'1' {
                c1 += len;
                last1 = len;
            } else {
                max0 = max0.max(len);
                if last0 > 0 {
                    min1 = min1.min(last1);
                    max = max.max(last0 + len);
                }
                last0 = len;
            }
            i = j;
        }
        c1 + max.max(if min1 > 0 { max0 - min1 } else { 0 })
    }
}