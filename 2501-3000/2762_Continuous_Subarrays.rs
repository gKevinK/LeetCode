impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut sl = std::collections::VecDeque::new();
        let mut ss = std::collections::VecDeque::new();
        let mut left = 0;
        let mut res = 0;
        for i in 0..nums.len() {
            let num = nums[i];
            while sl.back().map_or(i32::MAX, |&x| nums[x]) <= num {
                sl.pop_back();
            }
            sl.push_back(i);
            while ss.back().map_or(i32::MIN, |&x| nums[x]) >= num {
                ss.pop_back();
            }
            ss.push_back(i);
            while nums[*sl.front().unwrap()] - nums[*ss.front().unwrap()] > 2 {
                let slf = *sl.front().unwrap();
                let ssf = *ss.front().unwrap();
                if slf < ssf {
                    sl.pop_front();
                    left = slf + 1;
                } else {
                    ss.pop_front();
                    left = ssf + 1;
                }
            }
            res += (i - left + 1) as i64;
        }
        res
    }
}