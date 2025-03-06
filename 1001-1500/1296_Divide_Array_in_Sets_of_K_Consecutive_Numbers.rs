impl Solution {
    pub fn is_possible_divide(mut nums: Vec<i32>, k: i32) -> bool {
        if nums.len() as i32 % k != 0 {
            return false;
        }
        nums.sort();
        let mut q = std::collections::VecDeque::new();
        let mut i = 0;
        let mut cur = 0;
        while i < nums.len() {
            if q.is_empty() {
                cur = nums[i];
            }
            let mut n = 0;
            while nums.get(i) == Some(&cur) {
                i += 1;
                n += 1;
            }
            if n < q.len() {
                return false;
            }
            for _ in q.len()..n {
                q.push_back(cur + k - 1);
            }
            while q.front() == Some(&cur) {
                q.pop_front();
            }
            cur += 1;
        }
        q.is_empty()
    }
}