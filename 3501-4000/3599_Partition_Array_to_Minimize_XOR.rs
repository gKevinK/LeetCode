impl Solution {
    pub fn min_xor(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let ku = k as usize;
        let mut xor = vec![0; n + 1];
        for i in 0..n {
            xor[i + 1] = xor[i] ^ nums[i];
        }
        let mut dp1 = xor.clone();
        let mut dp2 = vec![i32::MAX; n + 1];
        for ik in 1..ku {
            for i in (ik + 1)..=(n - ku + ik + 1) {
                let mut x = i32::MAX;
                for j in ik..i {
                    x = x.min(dp1[j].max(xor[i] ^ xor[j]));
                }
                dp2[i] = x;
            }
            std::mem::swap(&mut dp1, &mut dp2);
            dp2.fill(i32::MAX);
        }
        dp1[n]
    }
}