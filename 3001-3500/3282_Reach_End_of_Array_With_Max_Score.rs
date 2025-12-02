impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut last = 0;
        let n = nums.len();
        for i in 0..n {
            if nums[i] > nums[last] {
                res += (i - last) as i64 * nums[last] as i64;
                last = i;
            }
        }
        res + nums[last] as i64 * (n - 1 - last) as i64
    }
}