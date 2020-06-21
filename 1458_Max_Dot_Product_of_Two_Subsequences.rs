impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::cmp::max;
        let m = nums1.len();
        let n = nums2.len();
        let mut dp = vec![vec![-1_000_000_000; n]; m];
        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    dp[i][j] = max(dp[i][j], dp[i - 1][j]);
                }
                if j > 0 {
                    dp[i][j] = max(dp[i][j], dp[i][j - 1]);
                }
                if i > 0 && j > 0 {
                    dp[i][j] = max(dp[i][j], dp[i - 1][j - 1] + nums1[i] * nums2[j]);
                }
                dp[i][j] = max(dp[i][j], nums1[i] * nums2[j]);
            }
        }
        dp[m - 1][n - 1]
    }
}