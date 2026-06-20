impl Solution {
    pub fn process_str(s: String, mut k: i64) -> char {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut len = 0i64;
        for &b in bytes.iter() {
            match b {
                b'*' => {
                    len = (len - 1).max(0);
                }
                b'#' => {
                    len *= 2;
                }
                b'%' => {}
                _ => {
                    len += 1;
                }
            }
        }
        if k >= len {
            return '.';
        }
        for &b in bytes.iter().rev() {
            match b {
                b'*' => {
                    len += 1;
                }
                b'#' => {
                    len /= 2;
                    if k >= len {
                        k -= len;
                    }
                }
                b'%' => {
                    k = len - k - 1;
                }
                _ => {
                    len -= 1;
                    if k == len {
                        return char::from(b);
                    }
                }
            }
        }
        '.'
    }
}