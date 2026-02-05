impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = nums.iter().sum::<i32>();
        sum % k
    }
}