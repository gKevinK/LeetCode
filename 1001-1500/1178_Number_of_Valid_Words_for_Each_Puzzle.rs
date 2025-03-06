impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let a = 'a' as u8;
        let mut m = std::collections::HashMap::new();
        for w in &words {
            let mut b = 0;
            for c in w.chars() {
                b = b | (1 << (c as u8 - a));
            }
            *m.entry(b).or_insert(0) += 1;
        }
        let mut r = Vec::new();
        for p in &puzzles {
            let mut mask = 0;
            for c in p.chars() {
                mask = mask | (1 << (c as u8 - a));
            }
            let mut n = 0;
            let mut first = 1 << (p.chars().next().unwrap() as u8 - a);
            let mut sub = mask;
            while sub > 0 {
                if first & sub > 0 {
                    n += m.get(&sub).unwrap_or(&0);
                }
                sub = (sub - 1) & mask;
            }
            r.push(n);
        }
        r
    }
}