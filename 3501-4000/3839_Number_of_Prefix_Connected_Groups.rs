impl Solution {
    pub fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        let k = k as usize;
        for word in &words {
            if word.len() >= k {
                let prefix = &word[..k];
                *map.entry(prefix).or_insert(0) += 1;
            }
        }
        map.values().map(|&v| i32::from(v > 1)).sum()
    }
}