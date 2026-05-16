impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut count = vec![0; 26];
        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }
        count.sort();
        count.iter().take((26 - k) as usize).sum()
    }
}