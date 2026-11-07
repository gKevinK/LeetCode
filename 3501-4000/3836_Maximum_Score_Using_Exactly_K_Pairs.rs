impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let n = nums1.len();
        let m = nums2.len();
        let k = k as usize;
        let min = i64::MIN / 2;
        let mut dp1 = vec![0; (n + 1) * (m + 1)];
        let mut dp2 = vec![min; (n + 1) * (m + 1)];
        for ik in 1..=k {
            for i in ik..=n - k + ik {
                for j in ik..=m - k + ik {
                    let a = dp2[(i - 1) * (m + 1) + j];
                    let b = dp2[i * (m + 1) + (j - 1)];
                    let c = dp1[(i - 1) * (m + 1) + (j - 1)]
                        + nums1[i - 1] as i64 * nums2[j - 1] as i64;
                    dp2[i * (m + 1) + j] = a.max(b).max(c);
                }
            }
            std::mem::swap(&mut dp1, &mut dp2);
            dp2.fill(min);
        }
        dp1[n * (m + 1) + m]
    }
}