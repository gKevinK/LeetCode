impl Solution {
    pub fn count_coprime(mat: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let m = mat.len();
        let n = mat[0].len();

        let mut mu = vec![0; 151];
        let mut min_prime = vec![0; 151];
        let mut primes = vec![];
        mu[1] = 1;
        for i in 2..=150 {
            if min_prime[i] == 0 {
                min_prime[i] = i;
                primes.push(i);
                mu[i] = -1;
            }
            for &p in &primes {
                if i * p > 150 {
                    break;
                }
                min_prime[i * p] = p;
                if i % p == 0 {
                    mu[i * p] = 0;
                    break;
                } else {
                    mu[i * p] = -mu[i];
                }
            }
        }

        let mut count = vec![vec![0; 151]; m];
        for i in 0..m {
            for &num in &mat[i] {
                count[i][num as usize] += 1;
            }
        }
        let mut div_count = vec![vec![0; 151]; m];
        for i in 0..m {
            for k in 1..=150 {
                for mul in (k..=150).step_by(k) {
                    div_count[i][k] += count[i][mul];
                }
            }
        }

        let mut res = 0;
        for k in 1..=150 {
            if mu[k] == 0 {
                continue;
            }
            let mut ways = 1;
            for i in 0..m {
                ways = (ways * div_count[i][k]) % MOD;
            }
            if mu[k] == 1 {
                res = (res + ways) % MOD;
            } else {
                res = (res - ways + MOD) % MOD;
            }
        }
        res as _
    }
}