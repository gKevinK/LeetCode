impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut r = 0;
        let mut dp = vec![0; m * n];
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    let mut width = if j == 0 { 1 } else { dp[i * n + j - 1] + 1 };
                    dp[i * n + j] = width;
                    for k in (0..=i).rev() {
                        width = width.min(dp[k * n + j]);
                        if width == 0 {
                            break;
                        }
                        r += width;
                    }
                }
            }
        }
        r
    }
}