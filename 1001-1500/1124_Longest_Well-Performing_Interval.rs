impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = vec![-1; 10001];
        let mut curr: i32 = 0;
        for i in 0..hours.len() {
            let d = if hours[i] > 8 { 1 } else { -1 };
            curr += d;
            if curr > 0 {
                res = std::cmp::max(res, i as i32 + 1);
            } else {
                if dp[(-curr + 1) as usize] != -1 {
                    res = std::cmp::max(res, i as i32 - dp[(-curr + 1) as usize]);
                }
                if d == -1 && dp[-curr as usize] == -1 {
                    dp[-curr as usize] = i as i32;
                }
            }
        }
        res
    }
}