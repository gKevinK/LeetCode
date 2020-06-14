impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut v = vec![0; 26];
        for c in s.chars() {
            v[c as usize - 'a' as usize] += 1;
        }
        if v.iter().sum::<i32>() < k {
            return false;
        }
        let n = v.iter().filter(|i| *i % 2 == 1).count() as i32;
        n <= k
    }
}