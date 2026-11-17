impl Solution {
    pub fn trim_trailing_vowels(s: String) -> String {
        let mut bytes = s.as_bytes();
        let mut p = bytes.len();
        while p > 0 {
            match bytes[p - 1] {
                b'a' | b'e' | b'i' | b'o' | b'u' => { p -= 1; },
                _ => { break; }
            }
        }
        String::from_utf8_lossy(&bytes[..p]).into_owned()
    }
}