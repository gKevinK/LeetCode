impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        for c in s.chars() {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                return true;
            }
        }
        false
    }
}