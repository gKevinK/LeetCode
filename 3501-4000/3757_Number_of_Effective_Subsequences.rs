const MOD: i64 = 1_000_000_007;
const LIMIT: usize = 100_001;

const pow2: [i64; LIMIT] = {
    let mut p = [1; LIMIT];
    let mut i = 1;
    while i < LIMIT {
        p[i] = (p[i - 1] * 2) % MOD;
        i += 1;
    }
    p
};

impl Solution {
    pub fn count_effective(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total_or = 0;
        for &x in &nums {
            total_or |= x;
        }
        if total_or == 0 {
            return 0;
        }

        let mut bit_map = Vec::with_capacity(20);
        for i in 0..20 {
            if (total_or >> i) & 1 == 1 {
                bit_map.push(i);
            }
        }
        let k = bit_map.len();
        let size = 1 << k;
        let mut dp = vec![0; size];

        for &x in &nums {
            let mut compressed_mask = 0;
            for (idx, &original_bit) in bit_map.iter().enumerate() {
                if (x >> original_bit) & 1 == 1 {
                    compressed_mask |= 1 << idx;
                }
            }
            dp[compressed_mask] += 1;
        }
        for i in 0..k {
            let bit = 1 << i;
            for mask in 0..size {
                if mask & bit != 0 {
                    dp[mask] += dp[mask ^ bit];
                }
            }
        }
        for i in 0..size {
            dp[i] = pow2[dp[i]] as usize; 
        }

        for i in 0..k {
            let bit = 1 << i;
            for mask in 0..size {
                if mask & bit != 0 {
                    let sub = dp[mask ^ bit];
                    let current = dp[mask];
                    if current >= sub {
                        dp[mask] = current - sub;
                    } else {
                        dp[mask] = (current as i64 - sub as i64 + MOD) as usize;
                    }
                }
            }
        }

        let count_eq_total = dp[size - 1] as i64;
        let total_subsets = pow2[n];
        ((total_subsets - count_eq_total + MOD) % MOD) as _
    }
}