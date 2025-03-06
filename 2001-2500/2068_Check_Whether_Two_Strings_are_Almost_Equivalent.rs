impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut count = vec![0i32; 26];
        for c in word1.chars() {
            count[c as usize - 'a' as usize] += 1;
        }
        for c in word2.chars() {
            count[c as usize - 'a' as usize] -= 1;
        }
        count.iter().all(|&n| n.abs() <= 3)
    }
}