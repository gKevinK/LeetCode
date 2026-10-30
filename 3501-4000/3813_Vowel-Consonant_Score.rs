impl Solution {
    pub fn vowel_consonant_score(s: String) -> i32 {
        let mut v = 0;
        let mut c = 0;
        for ch in s.chars() {
            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' => { v += 1 },
                ' ' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {},
                _ => { c += 1 },
            }
        }
        if c > 0 { v / c } else { 0 }
    }
}