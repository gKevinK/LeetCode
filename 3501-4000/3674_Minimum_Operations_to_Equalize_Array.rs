impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n0 = nums[0];
        if nums.iter().all(|&x| x == n0) { 0 } else { 1 }
    }
}