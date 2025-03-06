impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let mut r = Vec::new();
        let mut row = 0;
        let mut len = 0;
        for c in s.chars() {
            if c == ' ' {
                row = 0;
                len += 1;
            } else {
                if r.len() <= row {
                    r.push(String::new());
                }
                while r[row].len() < len {
                    r[row].push(' ');
                }
                r[row].push(c);
                row += 1;
            }
        }
        r
    }
}