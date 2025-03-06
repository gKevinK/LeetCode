impl Solution {
    fn test(nums: &Vec<i32>, k: i32) -> i64 {
        let mut v = vec![0; 100001];
        let mut res = 0;
        let mut len = 0;
        let mut left = 0;
        for right in 0..nums.len() {
            if v[nums[right] as usize] == 0 {
                len += 1;
            }
            v[nums[right] as usize] += 1;
            while len > k {
                v[nums[left] as usize] -= 1;
                if v[nums[left] as usize] == 0 {
                    len -= 1;
                }
                left += 1;
            }
            res += (right - left + 1) as i64
        }
        res
    }

    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut s = HashSet::new();
        for n in &nums {
            s.insert(*n);
        }
        let size = (nums.len() as i64) * (nums.len() as i64 + 1) / 2;
        let mut lo = 1i32;
        let mut hi = s.len() as i32;
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            let n = Self::test(&nums, mi);
            if n * 2 < size {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        lo
    }
}