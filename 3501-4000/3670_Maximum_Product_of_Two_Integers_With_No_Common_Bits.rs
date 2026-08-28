impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let max = *nums.iter().max().unwrap();
        let bits = max.ilog2();
        let mask = (1 << (bits + 1)) - 1;
        let mut dp = vec![0; mask + 1];
        for &num in &nums {
            dp[num as usize] = num;
        }
        for b in 0..=bits {
            for m in 0..mask {
                let m2 = m ^ (1 << b);
                if m > m2 {
                    dp[m] = dp[m].max(dp[m2]);
                }
            }
        }
        let mut res = 0;
        for &num in &nums {
            res = res.max(num as i64 * dp[mask ^ num as usize] as i64);
        }
        res
    }
}