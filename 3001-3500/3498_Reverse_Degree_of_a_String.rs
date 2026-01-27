impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.bytes().enumerate().map(|(i, b)| (i as i32 + 1) * (b'z' - b + 1) as i32).sum()
    }
}