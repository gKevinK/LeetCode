impl Solution {
    pub fn lexicographically_smallest_string(s: String) -> String {
        let mut s = s.as_bytes().to_vec();
        let n = s.len();
        let adj = |a: u8, b: u8| {
            let diff = a.abs_diff(b);
            diff == 1 || diff == 25
        };
        let mut rm = vec![vec![false; n + 1]; n + 1];
        for i in (0..n).rev() {
            rm[i][i] = true;
            for j in (i + 2..=n).step_by(2) {
                if rm[i + 1][j - 1] && adj(s[i], s[j - 1]) {
                    rm[i][j] = true;
                    continue;
                }
                for k in (i + 2..j).step_by(2) {
                    if rm[i][k] && rm[k][j] {
                        rm[i][j] = true;
                        break;
                    }
                }
            }
        }
        let mut dp = vec![String::new(); n + 1];
        for i in (0..n).rev() {
            dp[i] = String::from(s[i] as char) + &dp[i + 1];
            for j in (i + 2..=n).step_by(2) {
                if rm[i][j] && dp[j] < dp[i] {
                    dp[i] = dp[j].clone();
                }
            }
        }
        dp[0].clone()
    }
}