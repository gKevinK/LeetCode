impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut r = (k - nums[0]).abs();
        let mut left = 0;
        for right in 0..nums.len() {
            if right > 0 {
                while left < right && nums[left] & !nums[left+1] == 0 {
                    left += 1;
                }
            }
            for i in left..(right+1) {
                nums[i] = nums[i] | nums[right];
                r = std::cmp::min(r, (k - nums[i]).abs());
                if r == 0 {
                    return 0;
                }
            }
        }
        r
    }
}