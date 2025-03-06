impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut v = vec![0; 26];
        for c in s.chars() {
            v[c as usize - 'a' as usize] += 1;
        }
        for c in t.chars() {
            v[c as usize - 'a' as usize] -= 1;
        }
        let mut r = 0;
        for i in v {
            if i > 0 {
                r += i;
            }
        }
        r
    }
}