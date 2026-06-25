impl Solution {
    pub fn maximum_median_sum(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = 0;
        for i in 1..=(n / 3) {
            res += nums[n - i * 2] as i64;
        }
        res
    }
}