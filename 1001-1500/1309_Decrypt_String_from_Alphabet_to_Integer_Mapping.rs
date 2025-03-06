impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let v: Vec<char> = s.chars().collect();
        let mut r = String::from("");
        let mut i = 0;
        while i < v.len() {
            if v.get(i + 2) == Some(&'#') {
                let n = (v[i] as u8 - '0' as u8) * 10 + v[i + 1] as u8 - '0' as u8;
                r.push(('a' as u8 + n - 1) as char);
                i += 3;
            } else {
                r.push(('a' as u8 + v[i] as u8 - '1' as u8) as char);
                i += 1;
            }
        }
        r
    }
}