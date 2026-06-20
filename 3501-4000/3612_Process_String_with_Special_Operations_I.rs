impl Solution {
    pub fn process_str(s: String) -> String {
        let mut r = vec![];
        for &b in s.as_bytes() {
            if b'a' <= b && b <= b'z' {
                r.push(b);
            } else if b == b'*' {
                if !r.is_empty() {
                    r.pop();
                }
            } else if b == b'#' {
                let r2 = r.clone();
                r.extend_from_slice(&r2[..]);
            } else if b == b'%' {
                r.reverse();
            }
        }
        String::from_utf8(r).unwrap()
    }
}