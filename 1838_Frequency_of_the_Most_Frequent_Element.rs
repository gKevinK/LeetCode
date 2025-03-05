impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort();
        let mut m = 0;
        let mut lo = nums.len() - 1;
        for hi in (0..nums.len()).rev() {
            if hi + 1 < nums.len() {
                k += (nums[hi + 1] - nums[hi]) * (hi + 1 - lo) as i32;
            }
            while lo > 0 && nums[hi] - nums[lo - 1] <= k {
                lo -= 1;
                k -= nums[hi] - nums[lo];
            }
            m = std::cmp::max(m, (hi + 1 - lo) as i32);
        }
        m
    }
}