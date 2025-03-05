impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut avg = vec![-1; nums.len()];
        if k * 2 >= nums.len() as i32 {
            return avg;
        }
        let mut sum = 0i64;
        for i in 0..=(k * 2) {
            sum += nums[i as usize] as i64;
        }
        let len = (k * 2 + 1) as i64;
        avg[k as usize] = (sum / len) as i32;
        for i in (k + 1)..(nums.len() as i32 - k) {
            sum += nums[(i + k) as usize] as i64;
            sum -= nums[(i - k - 1) as usize] as i64;
            avg[i as usize] = (sum / len) as i32;
        }
        avg
    }
}