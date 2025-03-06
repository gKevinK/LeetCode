impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let s = s.into_bytes();
        let n_sqrt = (n as f64).sqrt() as usize + 1;
        let mut res = 0;
    
        let mut i = 0;
        while i < n {
            if s[i] == b'0' {
                i += 1;
                continue;
            }
            let mut len = 0;
            while i < n && s[i] == b'1' {
                len += 1;
                i += 1;
            }
            res += (len * (len + 1)) / 2;
        }
    
        for n0 in 1..n_sqrt {
            let mut l = 0;
            let mut r = 0;
            let mut cl1 = 0;
            while l + cl1 < n && s[l + cl1] == b'1' {
                cl1 += 1;
            }
            let mut c0 = 0;
            let mut c1 = 0;
            while r < n {
                if s[r] == b'0' {
                    c0 += 1;
                    if c0 > n0 {
                        l += cl1 + 1;
                        c0 -= 1;
                        c1 -= cl1;
                        cl1 = 0;
                        while l + cl1 < n && s[l + cl1] == b'1' {
                            cl1 += 1;
                        }
                    }
                } else {
                    c1 += 1;
                }
                if c0 == n0 && c1 >= n0.pow(2) {
                    res += std::cmp::min(cl1 + 1, c1 - n0.pow(2) + 1);
                }
                r += 1;
            }
        }
        res as i32
    }
}