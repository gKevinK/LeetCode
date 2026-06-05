impl Solution {
    pub fn resulting_string(s: String) -> String {
        let mut v: Vec<u8> = Vec::with_capacity(s.len());
        for x in s.bytes() {
            if let Some(b) = v.last() {
                let d = b.abs_diff(x);
                if d == 1 || d == 25 {
                    v.pop();
                } else {
                    v.push(x);
                }
            } else {
                v.push(x);
            }
        }
        String::from_utf8(v).unwrap()
    }
}