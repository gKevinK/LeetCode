impl Solution {
    pub fn count_stable_subsequences(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut x00 = 0;
        let mut x01 = 0;
        let mut x10 = 0;
        let mut x11 = 0;
        let mut xx0 = 0;
        let mut xx1 = 0;
        for &num in &nums {
            if num % 2 == 0 {
                x00 = (x00 + x10 + xx0) % MOD;
                x10 = (x10 + x01 + x11 + xx1) % MOD;
                xx0 += 1;
            } else {
                x11 = (x11 + x01 + xx1) % MOD;
                x01 = (x01 + x00 + x10 + xx0) % MOD;
                xx1 += 1;
            }
        }
        ((x00 + x01 + x10 + x11 + xx0 + xx1) % MOD) as _
    }
}