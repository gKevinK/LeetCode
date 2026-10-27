impl Solution {
    pub fn min_operations(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for i in 0..nums.len() {
            if nums[i] != target[i] {
                set.insert(nums[i]);
            }
        }
        set.len() as _
    }
}