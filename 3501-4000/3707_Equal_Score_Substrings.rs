impl Solution {
    pub fn score_balance(s: String) -> bool {
        let bytes = s.as_bytes();
        let sum = bytes.iter().map(|&b| (b - b'a' + 1) as i32).sum::<i32>();
        let mut s1 = 0;
        for b in bytes {
            s1 += (b - b'a' + 1) as i32;
            if s1 * 2 == sum {
                return true;
            }
        }
        false
    }
}