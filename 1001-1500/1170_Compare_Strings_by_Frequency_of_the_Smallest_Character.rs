impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut freq = vec![0; 12];
        for w in &words {
            let mut f = 0;
            let mut c = 'z';
            for ch in w.chars() {
                if (ch < c) {
                    c = ch;
                    f = 1;
                } else if (ch == c) {
                    f += 1;
                }
            }
            freq[f] += 1;
        }
        for i in (0..10).rev() {
            freq[i] += freq[i + 1];
        }
        for q in &queries {
            let mut f = 0;
            let mut c = 'z';
            for ch in q.chars() {
                if (ch < c) {
                    c = ch;
                    f = 1;
                } else if (ch == c) {
                    f += 1;
                }
            }
            res.push(freq[f + 1]);
        }
        res
    }
}