impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let str1: Vec<_> = str1.chars().collect();
        let str2: Vec<_> = str2.chars().collect();
        let mut dp = vec![vec![0; str2.len() + 1]; str1.len() + 1];
        for i in 1..=str1.len() {
            for j in 1..=str2.len() {
                if str1[i - 1] == str2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i][j - 1], dp[i - 1][j]);
                }
            }
        }
        let mut res = String::new();
        let mut i = str1.len();
        let mut j = str2.len();
        while i != 0 || j != 0 {
            if i > 0 && dp[i][j] == dp[i - 1][j] {
                i -= 1;
                res.push(str1[i]);
            } else if j > 0 && dp[i][j] == dp[i][j - 1] {
                j -= 1;
                res.push(str2[j]);
            } else {
                i -= 1;
                j -= 1;
                res.push(str1[i]);
            }
        }
        res.chars().rev().collect()
    }
}