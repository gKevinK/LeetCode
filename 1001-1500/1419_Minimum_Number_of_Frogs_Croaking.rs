impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut dp = vec![0; 5];
        for c in croak_of_frogs.chars() {
            let i = match c {
                'c' => 0,
                'r' => 1,
                'o' => 2,
                'a' => 3,
                'k' => 4,
                _ => 100,
            };
            if i >= 1 && dp[i - 1] == 0 {
                return -1;
            }
            if i == 0 && dp[4] > 0 {
                dp[4] -= 1;
            }
            if i >= 1 {
                dp[i - 1] -= 1;
            }
            dp[i] += 1;
        }
        if dp.iter().take(4).any(|x| *x > 0) {
            return -1;
        }
        dp[4]
    }
}