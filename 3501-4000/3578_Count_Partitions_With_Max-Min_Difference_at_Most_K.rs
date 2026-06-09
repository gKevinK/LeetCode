impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let MOD = 1_000_000_007;
        let mut n = nums.len();
        use std::collections::VecDeque;
        let mut sl = VecDeque::with_capacity(n);
        let mut ss = VecDeque::with_capacity(n);
        let mut dp = vec![0; n + 1];
        let mut sum = 1;
        let mut left = 0;
        dp[0] = 1;
        for i in 0..n {
            let num = nums[i];
            while sl.back().map_or(i32::MAX, |i| nums[*i]) <= num {
                sl.pop_back();
            }
            sl.push_back(i);
            while ss.back().map_or(i32::MIN, |i| nums[*i]) >= num {
                ss.pop_back();
            }
            ss.push_back(i);
            while nums[*sl.front().unwrap()] as i64 - nums[*ss.front().unwrap()] as i64 > k as i64 {
                sum = (sum + MOD - dp[left]) % MOD;
                left += 1;
                if *sl.front().unwrap() < left {
                    sl.pop_front();
                }
                if *ss.front().unwrap() < left {
                    ss.pop_front();
                }
            }
            dp[i + 1] = sum;
            sum = (sum * 2) % MOD;
        }
        dp[n]
    }
}