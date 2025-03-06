impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;
        let k_ = k as usize;
        while i + k_ <= s.len() {
            res.push(String::from(&s[i..i + k_]));
            i += k_;
        }
        if i < s.len() {
            let mut last = String::from(&s[i..]);
            while last.len() < k_ {
                last.push(fill);
            }
            res.push(last);
        }
        res
    }
}