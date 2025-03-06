impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut mark = vec![0; s.len()];
        let mut stack = vec![];
        for (i, c) in s.chars().enumerate() {
            if c.is_ascii_digit() {
                mark[i] = 1;
                mark[stack.pop().unwrap()] = 1;
            } else {
                stack.push(i);
            }
        }
        let mut res = String::new();
        for (i, c) in s.chars().enumerate() {
            if mark[i] == 0 {
                res.push(c);
            }
        }
        res
    }
}