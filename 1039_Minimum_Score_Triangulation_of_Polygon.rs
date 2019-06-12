impl Solution {
    pub fn min_score_triangulation(a: Vec<i32>) -> i32 {
        let mut n = a.len() as i32;
        let mut dp = vec![vec![0; a.len()]; a.len()];
        for l in 2..n as usize {
            for i in 0..(n-l as i32) as usize {
                let j = i + l;
                let mut r1 = std::i32::MAX;
                for k in (i+1)..j {
                    let mut r2 = a[i] * a[j] * a[k];
                    if i + 1 < k {
                        r2 += dp[i][k];
                    }
                    if k + 1 < j {
                        r2 += dp[k][j];
                    }
                    r1 = std::cmp::min(r1, r2);
                }
                dp[i][j] = r1;
            }
        }
        dp[0][(n - 1) as usize]
    }
}