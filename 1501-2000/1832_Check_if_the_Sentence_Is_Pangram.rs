impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut rest = 26;
        let mut count = vec![0; 26];
        for c in sentence.chars() {
            if count[c as usize - 'a' as usize] == 0 {
                rest -= 1;
                count[c as usize - 'a' as usize] = 1;
                if rest == 0 {
                    return true;
                }
            }
        }
        false
    }
}