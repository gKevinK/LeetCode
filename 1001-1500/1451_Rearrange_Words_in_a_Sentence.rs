impl Solution {
    pub fn arrange_words(text: String) -> String {
        let mut words: Vec<_> = text.split_ascii_whitespace().collect();
        let w1 = words[0].to_lowercase();
        words[0] = &w1;
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut c = words[0].chars();
        let w2 = c.next().unwrap().to_uppercase().chain(c).collect::<String>();
        words[0] = &w2;
        words.join(" ")
    }
}