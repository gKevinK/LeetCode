impl Solution {
    pub fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
        reward_values.sort_unstable();
        let max_value = *reward_values.last().unwrap();
        let mut dp = vec![false; max_value as usize + 1];
        dp[0] = true;
        let mut max = 0;
        for i in 0..(reward_values.len() - 1) {
            if i == 0 || reward_values[i - 1] != reward_values[i] {
                let limit = std::cmp::min(reward_values[i], max_value - reward_values[i]);
                for x in 0..limit as usize {
                    if dp[x] {
                        dp[reward_values[i] as usize + x] = true;
                        max = max.max(reward_values[i] + x as i32);
                    }
                }
            }
        }
        max_value + max
    }
}