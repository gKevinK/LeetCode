impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let t1: Vec<char> = text1.chars().collect();
        let t2: Vec<char> = text2.chars().collect();
        let mut dp = vec![0; t2.len() + 1];
        for c1 in t1 {
            for i in (0..t2.len()).rev() {
                if t2[i] == c1 {
                    dp[i + 1] = dp[i] + 1;
                }
            }
            for i in 0..t2.len() {
                if t2[i] != c1 {
                    dp[i + 1] = std::cmp::max(dp[i], dp[i + 1]);
                }
            }
        }
        dp[t2.len()]
    }
}