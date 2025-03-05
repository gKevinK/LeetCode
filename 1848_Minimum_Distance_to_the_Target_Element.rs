impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let ustart = start as usize;
        for i in 0..nums.len() {
            if i <= ustart && nums[ustart - i] == target {
                return i as i32;
            }
            if ustart + i < nums.len() && nums[ustart + i] == target {
                return i as i32;
            }
        }
        -1
    }
}