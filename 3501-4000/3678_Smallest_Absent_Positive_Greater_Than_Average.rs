impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let sum = nums.iter().sum();
        let mut dp = [0; 202];
        for &num in &nums {
            dp[num as usize + 100] += 1;
        }
        for x in 1..=101 {
            if x * (n as i32) > sum && dp[x as usize + 100] == 0 {
                return x;
            }
        }
        101
    }
}