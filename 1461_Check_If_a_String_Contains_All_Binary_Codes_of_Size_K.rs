impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut v = vec![false; 1 << k];
        let mut m = !(1 << k);
        let mut n = 0;
        let mut l = 0;
        for c in s.chars() {
            n = ((n << 1) & m) + if c == '1' { 1 } else { 0 };
            l += 1;
            if l >= k {
                v[n as usize] = true;
            }
        }
        v.iter().all(|i| *i)
    }
}