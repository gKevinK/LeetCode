impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut count = [0; 26];
        for byte in bytes {
            count[(byte - b'a') as usize] += 1;
        }
        count.into_iter().map(|c| if c > 0 { 2 - c % 2 } else { 0 }).sum()
    }
}