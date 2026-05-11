impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut v = vec![0; 26];
        for c in s.chars() {
            v[c as usize - 'a' as usize] += 1;
        }
        let mut cv = 0;
        let mut cc = 0;
        for i in 0..26 {
            match ('a' as u8 + i) as char {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    cv = cv.max(v[i as usize]);
                },
                _ => {
                    cc = cc.max(v[i as usize]);
                }
            }
        }
        cv + cc
    }
}