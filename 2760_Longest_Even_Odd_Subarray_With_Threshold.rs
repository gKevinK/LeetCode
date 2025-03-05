impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut result = 0;
        let mut left = 0;
        for i in 0..nums.len() {
            if nums[i] > threshold {
                left = i + 1;
                continue;
            }
            if left != i {
                if nums[i] % 2 == nums[i - 1] % 2 {
                    if nums[i] % 2 == 0 {
                        left = i;
                    } else {
                        left = i + 1;
                        continue;
                    }
                }
            } else {
                if nums[i] % 2 != 0 {
                    left = i + 1;
                    continue;
                }
            }
            result = std::cmp::max(result, (i + 1 - left) as i32);
        }
        result
    }
}