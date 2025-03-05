impl Solution {
    pub fn longest_palindromic_subsequence(s: String, k: i32) -> i32 {
        use std::cmp::min;
        use std::cmp::max;

        let mut v = vec![];
        for b in s.bytes() {
            v.push((b - 'a' as u8) as i32);
        }
        let n = v.len();
        let mut dp = vec![];
        dp.push(vec![vec![0; n + 1]; n]);
        dp.push(vec![vec![0; n + 1]; n]);
        for i in 0..n {
            dp[1][i][i] = 999;
        }
        loop {
            dp.push(vec![vec![999; n + 1]; n]);
            let mut flag = false;
            let l = dp.len() - 1;
            for i1 in (0..(n - l + 1)).rev() {
                for i2 in (i1 + l)..=n {
                    dp[l][i1][i2] = min(dp[l][i1 + 1][i2], dp[l][i1][i2 - 1]);
                    let mi = dp[l - 2][i1 + 1][i2 - 1];
                    if mi > k {
                        continue;
                    }
                    let c1 = min(v[i1], v[i2 - 1]);
                    let c2 = max(v[i1], v[i2 - 1]);
                    let op = min(c2 - c1, c1 + 26 - c2);
                    if mi + op <= k {
                        flag = true;
                        dp[l][i1][i2] = min(dp[l][i1][i2], mi + op);
                    }
                }
            }
            if !flag {
                break;
            }
        }
        dp.len() as i32 - 2
    }
}