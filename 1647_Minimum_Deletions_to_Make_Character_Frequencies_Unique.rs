impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut v = vec![0; 26];
        for c in s.chars() {
            v[c as usize - 'a' as usize] += 1;
        }
        v.sort();
        let mut r = 0;
        let mut l = 1_000_000;
        for i in v.into_iter().rev() {
            if l == 0 {
                r += i;
            } else if i >= l {
                r += i - l + 1;
                l -= 1;
            } else {
                l = i;
            }
        }
        r
    }
}