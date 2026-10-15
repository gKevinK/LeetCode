impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut v = s.bytes().collect::<Vec<_>>();
        let n = v.len();

        let is_vowel = |b: u8| {
            match b {
                b'a' | b'e' | b'i' | b'o' | b'u' => true,
                _ => false
            }
        };
        let mut i = 0;
        let mut j = i;
        let mut vowel = 0;
        while j < n && v[j] != b' ' {
            vowel += i32::from(is_vowel(v[j]));
            j += 1;
        }
        i = j + 1;
        while i < n {
            let mut vowel2 = 0;
            j = i;
            while j < n && v[j] != b' ' {
                vowel2 += i32::from(is_vowel(v[j]));
                j += 1;
            }
            if vowel == vowel2 {
                v[i..j].reverse();
            }
            i = j + 1;
        }
        String::from_utf8(v).unwrap()
    }
}