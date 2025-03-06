impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut r = 0;
        let mut dp = vec![vec![0; n]; n];
        for i in 0..m {
            for j in 0..n {
                let mut f = true;
                for k in 0..=j {
                    if mat[i][j - k] == 0 {
                        f = false;
                    };
                    if f {
                        dp[j][k] += 1;
                        r += dp[j][k];
                    } else {
                        dp[j][k] = 0;
                    };
                }
            }
        }
        r
    }
}