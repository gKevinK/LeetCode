impl Solution {
    pub fn max_sum_two_no_overlap(a: Vec<i32>, l: i32, m: i32) -> i32 {
        use std::cmp::max;
        
        let mut sum: Vec<i32> = vec![0; a.len() + 1];
        for i in 0..a.len() {
            sum[i + 1] = sum[i] + a[i];
        }
        let mut dp: Vec<i32> = vec![0; a.len() + 1];
        for ii in (l..=(a.len() as i32 - m)).rev() {
            let i = ii as usize;
            dp[i] = max(dp[i + 1], sum[i + m as usize] - sum[i]);
        }
        let mut r = 0;
        let mut lm = 0;
        for ii in 0..=(a.len() as i32 - l) {
            let i = ii as usize;
            let mut mm = 0;
            if i >= m as usize {
                lm = max(lm, sum[i] - sum[(i as i32 - m) as usize]);
                mm = max(mm, lm);
            }
            if i + ((m + l) as usize) <= a.len() {
                mm = max(mm, dp[i + l as usize]);
            }
            r = max(r, mm + sum[i + l as usize] - sum[i]);
        }
        r
    }
}