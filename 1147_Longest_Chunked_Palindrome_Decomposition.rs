impl Solution {
    fn eq(chars: &Vec<char>, l1: usize, l2: usize, len: usize) -> bool {
        for i in 0..len {
            if chars[l1 + i] != chars[l2 + i] {
                return false;
            }
        }
        true
    }
    pub fn longest_decomposition(text: String) -> i32 {
        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();
        let mut start = 0;
        let mut r = 1;
        for i in (start + 1)..=(n / 2) {
            if Self::eq(&chars, start as usize, (n - i) as usize, (i - start) as usize) {
                r += if (i * 2 == n) { 1 } else { 2 };
                start = i;
            }
        }
        r
    }
}