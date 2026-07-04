impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let mut max = i64::MIN;
        let n = nums.len();
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }
        let mut left = 0;
        let mut p = 0;
        while left + 2 < n {
            if nums[left] >= nums[left + 1] {
                left += 1;
                continue;
            }
            p = p.max(left);
            while p + 1 < n && nums[p] < nums[p + 1] {
                p += 1;
            }
            if left == p {
                left = p + 1;
                continue;
            }
            let mut q = p;
            while q + 1 < n && nums[q] > nums[q + 1] {
                q += 1;
            }
            if p == q {
                left = q + 1;
                continue;
            }
            let mut r = q;
            while r + 1 < n && nums[r] < nums[r + 1] {
                r += 1;
            }
            if left < p && p < q && q < r && r < n {
                while nums[left] < 0 && left + 1 < p {
                    left += 1;
                }
                let s1 = sum[q + 2] - sum[left];
                max = max.max(s1);
                let s2 = sum[r + 1] - sum[left];
                max = max.max(s2);
            }
            left = q;
            p = r;
        }
        max
    }
}