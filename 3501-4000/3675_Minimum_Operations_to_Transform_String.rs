impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut na = 26;
        for b in s.bytes() {
            if b != b'a' {
                na = na.min(b - b'a');
            }
        }
        (26 - na) as i32
    }
}