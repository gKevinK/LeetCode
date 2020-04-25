impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = 0;
        for n in nums {
            sum += n;
            if sum < min {
                min = sum;
            }
        }
        1 - min
    }
}