impl Solution {
    pub fn max_substrings(word: String) -> i32 {
        let w = word.as_bytes();
        let n = w.len();
        let mut last = [n; 26];
        let mut res = 0;
        let mut mask = 0;
        for i in 0..n {
            let ord = (w[i] - b'a') as usize;
            if last[ord] == n {
                last[ord] = i;
            }
            if last[ord] + 3 <= i {
                res += 1;
                last.fill(n);
            }
        }
        res
    }
}