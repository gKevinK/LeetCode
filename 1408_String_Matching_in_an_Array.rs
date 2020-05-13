impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        for w in &words {
            if words.iter().any(|w2| w2.contains(w) && w != w2) {
                res.push(w.clone());
            }
        }
        res
    }
}