impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let is_vowel = |c| {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            }
        };
        let chars: Vec<_> = word.chars().collect();
        let mut result = 0;
        let mut n = chars.len() as i64;
        for i in 0..chars.len() {
            if is_vowel(chars[i]) {
                result += (i as i64 + 1) * (n - i as i64);
            }
        }
        result
    }
}