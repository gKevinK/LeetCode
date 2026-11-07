impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut sl = std::collections::VecDeque::new();
        let mut ss = std::collections::VecDeque::new();
        let mut left = 0;
        let mut res = 0;
        for (i, &v) in nums.iter().enumerate() {
            while sl.back().map_or(i32::MAX, |j| nums[*j]) <= v {
                sl.pop_back();
            }
            sl.push_back(i);
            while ss.back().map_or(0, |j| nums[*j]) >= v {
                ss.pop_back();
            }
            ss.push_back(i);
            let mut len = (i - left + 1) as i64;
            let mut l = nums[*sl.front().unwrap()];
            let mut s = nums[*ss.front().unwrap()];
            while (l - s) as i64 * len > k {
                if left == *sl.front().unwrap() {
                    sl.pop_front();
                    l = nums[*sl.front().unwrap()];
                }
                if left == *ss.front().unwrap() {
                    ss.pop_front();
                    s = nums[*ss.front().unwrap()];
                }
                left += 1;
                len -= 1;
            }
            res += len;
        }
        res
    }
}