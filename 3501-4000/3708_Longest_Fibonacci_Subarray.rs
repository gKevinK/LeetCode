impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 2;
        let mut res = 2;
        let n = nums.len();
        while j < n {
            if nums[j] == nums[j - 2] + nums[j - 1] {
                j += 1;
                res = res.max(j - i);
            } else {
                i = j - 1;
                j += 1;
            }
        }
        res as _
    }
}