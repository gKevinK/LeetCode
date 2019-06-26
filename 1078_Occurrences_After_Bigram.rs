impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let words: Vec<&str> = text.split(' ').collect();
        let mut r = Vec::new();
        let mut i = 0;
        while i + 2 < words.len() {
            if words[i] == first && words[i + 1] == second {
                r.push(String::from(words[i + 2]));
            }
            i += 1;
        }
        r
    }
}