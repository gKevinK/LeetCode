impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let mut used = [false; 26];
        for b in s.bytes() {
            let i = (b - b'a') as usize;
            used[i] = true;
        }
        used.iter().map(|&u| u as i32).sum()
    }
}