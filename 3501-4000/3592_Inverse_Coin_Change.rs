impl Solution {
    pub fn find_coins(num_ways: Vec<i32>) -> Vec<i32> {
        let n = num_ways.len();
        let mut set = vec![];
        let mut dp = [0; 101];
        dp[0] = 1;
        for i in 1..=n {
            let ways = num_ways[i - 1];
            if ways == dp[i] + 1 {
                set.push(i as i32);
                for k in 0..=(n - i) {
                    dp[k + i] += dp[k];
                }
            } else if ways != dp[i] {
                return vec![];
            }
        }
        set
    }
}