impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = n;
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while j < n && nums[i] as i64 * (k as i64) >= nums[j] as i64 {
                j += 1;
            }
            res = res.min(n - j + i);
            i += 1;
        }
        res as _
    }
}