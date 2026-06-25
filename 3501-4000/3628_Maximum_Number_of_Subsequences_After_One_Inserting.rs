impl Solution {
    pub fn num_of_subsequences(s: String) -> i64 {
        let n = s.len();
        let mut nt = s.bytes().filter(|&b| b == b'T').count() as i64;
        let mut res = 0;
        let mut nl = 0;
        let mut al = 0;
        let mut ac = 0;
        let mut at = 0;
        for (i, b) in s.bytes().enumerate() {
            ac = ac.max(nl * nt);
            if b == b'T' {
                nt -= 1;
            }
            if b == b'C' {
                res += nl * nt;
                al += nt;
                at += nl;
            }
            if b == b'L' {
                nl += 1;
            }
        }
        res + al.max(ac).max(at)
    }
}