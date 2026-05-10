impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let MOD = 1_000_000_007i64;
        let mu = m as usize;
        let ku = k as usize;
        let nu = nums.len();

        let flen = mu.max(nu);
        let mut fact = vec![1; flen + 1];
        let mut finv = vec![1; flen + 1];
        for i in 1..=flen {
            fact[i] = fact[i - 1] * (i as i64) % MOD;
        }
        finv[flen] = Self::pow_mod(fact[flen], MOD - 2, MOD);
        for i in (0..flen).rev() {
            finv[i] = finv[i + 1] * (i as i64 + 1) % MOD;
        }

        let mut pow = vec![vec![1; mu + 1]; nu];
        for i in 0..nu {
            for j in 1..=(mu) {
                pow[i][j] = pow[i][j - 1] * nums[i] as i64 % MOD;
            }
        }
        let index = |a: usize, b: usize, c: usize, d: usize| -> usize {
            ((a * (mu + 1) + b) * (mu / 2 + 1) + c) * (ku + 1) + d
        };
        let mut dp = vec![0; (nu + 1) * (mu + 1) * (mu / 2 + 1) * (ku + 1)];
        dp[index(0, mu, 0, ku)] = 1;
        for i in 0..nu {
            for lm in 0..=mu {
                for x in 0..=(mu / 2) {
                    for lk in 0..=ku {
                        let val = dp[index(i, lm, x, lk)];
                        if val == 0 {
                            continue;
                        }
                        for j in 0..=lm {
                            let bit = (x + j) & 1;
                            if bit <= lk {
                                let mut t = &mut dp[index(i + 1, lm - j, (x + j) >> 1, lk - bit)];
                                *t = ((val * pow[i][j] % MOD) * finv[j] % MOD + *t) % MOD;
                            }
                        }
                    }
                }
            }
        }
        let mut res = 0;
        for x in 0..=(mu / 2) {
            let count = x.count_ones() as usize;
            if count <= ku {
                res = (res + dp[index(nu, 0, x, count)]) % MOD;
            }
        }
        (res * fact[mu] % MOD) as i32
    }

    fn pow_mod(mut a: i64, mut p: i64, m: i64) -> i64 {
        let mut r = 1;
        while p > 0 {
            if p & 1 == 1 {
                r = r * a % m;
            }
            a = a * a % m;
            p >>= 1;
        }
        r
    }
}