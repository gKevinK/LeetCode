impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut res = 0;
        while left < nums.len() {
            if nums[left] == 0 {
                left += 1;
                continue;
            }
            let mut right = left;
            let mut neg = 0;
            let mut l = nums.len();
            let mut r = 0;
            while right < nums.len() && nums[right] != 0 {
                if nums[right] < 0 {
                    l = std::cmp::min(l, right);
                    r = std::cmp::max(r, right);
                    neg += 1;
                }
                right += 1;
            }
            res = std::cmp::max(res, {
                if neg % 2 == 0 {
                    right as i32 - left as i32
                } else {
                    std::cmp::max(right as i32 - l as i32 - 1, r as i32 - left as i32)
                }
            });
            left = right;
        }
        res
    }
}