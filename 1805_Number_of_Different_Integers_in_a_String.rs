impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut ints = std::collections::HashSet::new();
        let chars: Vec<_> = word.chars().collect();
        let n = chars.len();
        let mut i = 0;
        while i < n {
            if '0' as u8 <= chars[i] as u8 && chars[i] as u8 <= '9' as u8 {
                let mut j = i;
                while j < n && '0' as u8 <= chars[j] as u8 && chars[j] as u8 <= '9' as u8 {
                    j += 1;
                }
                while i + 1 < j && '0' == chars[i] {
                    i += 1;
                }
                ints.insert(String::from(&word[i..j]));
                i = j;
            } else {
                i += 1 ;
            }
        }
        ints.len() as i32
    }
}