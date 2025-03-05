impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut v = [0; 26];
        let mut r = 0;
        for (i, c) in s.as_bytes().iter().enumerate() {
            v[(c - 'a' as u8) as usize] = i as i32;
        }
        for (i, c) in t.as_bytes().iter().enumerate() {
            r += (v[(c - 'a' as u8) as usize] - i as i32).abs()
        }
        r
    }
}