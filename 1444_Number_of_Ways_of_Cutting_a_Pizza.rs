impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let MOD = 1_000_000_007;
        let mut vh = vec![vec![]; pizza.len()];
        let mut vv = vec![vec![]; pizza.len()];
        for (i, r) in pizza.iter().enumerate() {
            for c in r.chars() {
                vh[i].push(c == 'A');
                vv[i].push(c == 'A');
            }
        }
        let m = vh.len();
        let n = vh[0].len();
        for i in 0..m {
            let mut f = false;
            for j in (0..n).rev() {
                f = f || vh[i][j];
                vh[i][j] = f;
            }
        }
        for j in 0..n {
            let mut f = false;
            for i in (0..m).rev() {
                f = f || vv[i][j];
                vv[i][j] = f;
            }
        }
        let mut dp = vec![vec![vec![0; k as usize + 1]; n]; m];
        for i in (0..m).rev() {
            let mut f1 = false;
            for j in (0..n).rev() {
                f1 = f1 || vv[i][j];
                dp[i][j][1] = if f1 { 1 } else { 0 };
                if i + 1 < m {
                    let mut f = false;
                    for a in i..m {
                        if f {
                            for x in 2..=k as usize {
                                dp[i][j][x] += dp[a][j][x - 1];
                                dp[i][j][x] %= MOD;
                            }
                        }
                        f = f || vh[a][j];
                    }
                }
                if j + 1 < n {
                    let mut f = false;
                    for b in j..n {
                        if f {
                            for x in 2..=k as usize {
                                dp[i][j][x] += dp[i][b][x - 1];
                                dp[i][j][x] %= MOD;
                            }
                        }
                        f = f || vv[i][b];
                    }
                }
            }
        }
        dp[0][0][k as usize]
    }
}