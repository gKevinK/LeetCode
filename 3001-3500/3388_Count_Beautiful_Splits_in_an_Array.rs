impl Solution {
    pub fn beautiful_splits(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut z = vec![0; n];
        let mut z0 = vec![];
        for s1 in 0..n {
            let (mut l, mut r) = (0, 0);
            z[s1] = 0;
            for i in (s1 + 1)..n {
                z[i] = 0;
                let o = if r + 1 >= i { r + 1 - i } else { 0 };
                if i <= r {
                    z[i] = std::cmp::min(z[i - l + s1], o);
                }
                while i + z[i] < n && nums[s1 + z[i]] == nums[i + z[i]] {
                    z[i] += 1;
                }
                if i + z[i] - 1 > r {
                    (l, r) = (i, i + z[i] - 1);
                }
            }

            if s1 == 0 {
                z0 = z.clone();
            } else {
                for s2 in (s1 + 1)..n {
                    if z0[s1] >= s1 && s1 <= s2 - s1 {
                        res += 1;
                        continue;
                    }
                    if z[s2] >= s2 - s1 {
                        res += 1;
                        continue;
                    }
                }
            }
        }
        res
    }
}