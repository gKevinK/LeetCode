impl Solution {
    pub fn transform_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut count_even = 0;
        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                count_even += 1;
            }
        }
        for i in 0..nums.len() {
            nums[i] = if i < count_even { 0 } else { 1 };
        }
        nums
    }
}