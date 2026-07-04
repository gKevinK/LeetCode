impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut state = 0;
        if nums[0] >= nums[1] {
            return false;
        }
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                return false;
            }
            if state % 2 == 0 && nums[i] > nums[i - 1] {
                state += 1;
            } else if state % 2 == 1 && nums[i] < nums[i - 1] {
                state += 1;
            }
        }
        state == 3
    }
}