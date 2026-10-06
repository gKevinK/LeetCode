impl Solution {
    pub fn count_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut r = n - k as usize;
        while r > 0 && r < n && nums[r - 1] == nums[r] {
            r -= 1;
        }
        r as _
    }
}