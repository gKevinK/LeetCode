impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut res = std::i32::MAX;
        for len in l as usize..=r as usize {
            let mut s = 0;
            for i in 0..len {
                s += nums[i];
            }
            if s > 0 && s < res {
                res = s;
            }
            for i in 0..(nums.len() - len) {
                s += nums[i + len] - nums[i];
                if s > 0 && s < res {
                    res = s;
                }
            }
        }
        if res == std::i32::MAX { -1 } else { res }
    }
}