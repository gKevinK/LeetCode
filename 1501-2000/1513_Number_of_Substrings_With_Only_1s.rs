impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut MOD = 1_000_000_007;
        let mut r = 0;
        let mut l = 0;
        for c in s.chars() {
            if c == '1' {
                l += 1;
                r = (r + l) % MOD;
            } else {
                l = 0;
            }
        }
        r
    }
}