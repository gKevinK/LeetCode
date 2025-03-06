impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for w in words {
            let v: Vec<_> = w.chars().collect();
            let n = v.len();
            let mut p = true;
            for i in 0..(n / 2) {
                if v[i] != v[n - i - 1] {
                    p = false;
                    break;
                }
            }
            if p {
                return w;
            }
        }
        String::from("")
    }
}