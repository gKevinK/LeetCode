impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix = nums.clone();
        for i in 1..n {
            prefix[i] = prefix[i].max(prefix[i - 1]);
        }
        let mut suffix1 = i32::MAX;
        let mut res = prefix.clone();
        for i in (0..n - 1).rev() {
            suffix1 = suffix1.min(nums[i + 1]);
            if prefix[i] > suffix1 {
                res[i] = res[i + 1];
            }
        }
        res
    }
}