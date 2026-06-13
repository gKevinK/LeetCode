impl Solution {
    pub fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        let mut res = i64::MIN;
        for i in (m as usize - 1)..nums.len() {
            let j = i + 1 - m as usize;
            max = max.max(nums[j]);
            min = min.min(nums[j]);
            res = res.max(nums[i] as i64 * max as i64);
            res = res.max(nums[i] as i64 * min as i64);
        }
        res
    }
}