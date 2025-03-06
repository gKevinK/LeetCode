impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }
            let mut i2 = nums[i] + i as i32;
            if i2 < 0 {
                i2 += n as i32 * ((-i2) / n as i32 + 1);
            }
            i2 %= n as i32;
            result[i] = nums[i2 as usize];
        }
        result
    }
}