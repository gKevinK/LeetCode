impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if nums[i] == (i as i32 % 10) {
                return i as i32;
            }
        }
        -1
    }
}