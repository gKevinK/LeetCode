impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut v = Vec::new();
        for c in s.chars() {
            if let Some(&c2) = v.last() {
                if c == c2 {
                    v.pop();
                    continue;
                }
            }
            v.push(c);
        }
        v.iter().collect()
    }
}