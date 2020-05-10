impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut v1 = vec![0; 26];
        let mut v2 = vec![0; 26];
        for c in s1.chars() {
            v1[c as usize - 'a' as usize] += 1;
        }
        for c in s2.chars() {
            v2[c as usize - 'a' as usize] += 1;
        }
        let mut c1 = 0;
        let mut c2 = 0;
        let mut f1 = true;
        let mut f2 = true;
        for i in 0..26 {
            c1 += v1[i];
            c2 += v2[i];
            if c1 < c2 {
                f1 = false;
            }
            if c1 > c2 {
                f2 = false;
            }
        }
        f1 || f2
    }
}