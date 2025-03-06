impl Solution {
    pub fn longest_subarray(mut nums: Vec<i32>, limit: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::VecDeque;
        let mut res = 0;
        let mut left = 0;
        let mut v1 = VecDeque::<(i32, usize)>::new();
        let mut v2 = VecDeque::<(Reverse<i32>, usize)>::new();
        for (i, &n) in nums.iter().enumerate() {
            while !v1.is_empty() && v1.back().unwrap().0 > n {
                v1.pop_back();
            }
            while !v2.is_empty() && v2.back().unwrap().0 > Reverse(n) {
                v2.pop_back();
            }
            v1.push_back((n, i));
            v2.push_back((Reverse(n), i));
            while (v2.front().unwrap().0).0 - v1.front().unwrap().0 > limit {
                if v1.front().unwrap().1 < v2.front().unwrap().1 {
                    left = v1.pop_front().unwrap().1 + 1;
                } else {
                    left = v2.pop_front().unwrap().1 + 1;
                }
            }
            res = std::cmp::max(res, i - left + 1);
        }
        res as i32
    }
}