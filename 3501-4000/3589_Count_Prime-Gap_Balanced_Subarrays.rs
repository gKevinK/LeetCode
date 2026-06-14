const LIMIT: usize = 50001;

const PRIME: [bool; LIMIT] = {
    let mut p = [true; LIMIT];
    p[0] = false;
    p[1] = false;
    let mut i = 2;
    while i <= LIMIT.isqrt() + 1 {
        let mut j = i * i;
        while j < LIMIT {
            p[j] = false;
            j += i;
        }
        i += 1;
    }
    p
};

impl Solution {
    pub fn prime_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut sl = std::collections::VecDeque::new();
        let mut ss = std::collections::VecDeque::new();
        let mut left = 0;
        let mut p1 = -1;
        let mut p2 = -1;
        let mut res = 0;
        for i in 0..n {
            if PRIME[nums[i] as usize] {
                let num = nums[i];
                while sl.back().map_or(i32::MAX, |j| nums[*j]) <= num {
                    sl.pop_back();
                }
                sl.push_back(i);
                while ss.back().map_or(i32::MIN, |j| nums[*j]) >= num {
                    ss.pop_back();
                }
                ss.push_back(i);
                p1 = p2;
                p2 = i as i32;
                while nums[*sl.front().unwrap()] - nums[*ss.front().unwrap()] > k {
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
            }
            res += (p1 - left as i32 + 1).max(0);
        }
        res
    }
}