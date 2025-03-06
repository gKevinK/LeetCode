impl Solution {
    pub fn max_sum_after_partitioning(a: Vec<i32>, k: i32) -> i32 {
        use std::cmp::max;
        let mut dp = vec![0; a.len() + 1];
        let n = a.len() as i32;
        for i in 1..=n {
            let mut mx = 0;
            for j in (max(i - k, 0)..i).rev() {
                mx = max(mx, a[j as usize]);
                dp[i as usize] = max(dp[i as usize], dp[j as usize] + (i - j) * mx);
            }
        }
        dp[a.len()]
    }
}