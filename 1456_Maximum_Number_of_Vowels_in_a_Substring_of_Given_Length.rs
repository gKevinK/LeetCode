impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut m = 0;
        let mut c = 0;
        let mut v: Vec<_> = s.chars().collect();
        let mut i = 0;
        let mut j = 0;
        while j < v.len() {
            match v[j] {
                'a' | 'e' | 'i' | 'o' | 'u' => c += 1,
                _ => (),
            }
            if i + k as usize <= j {
                match v[i] {
                    'a' | 'e' | 'i' | 'o' | 'u' => c -= 1,
                    _ => (),
                }
                i += 1;
            }
            m = std::cmp::max(m, c);
            j += 1;
        }
        m
    }
}