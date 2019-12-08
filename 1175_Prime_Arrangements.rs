impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let MOD = 1_000_000_007;
        let mut v = vec![true; n as usize + 1];
        for i in 2..n {
            if v[i as usize] {
                let mut x = i * 2;
                while x <= n {
                    v[x as usize] = false;
                    x += i;
                }
            }
        }
        let mut p = 0;
        for i in 2..=n {
            p += if v[i as usize] { 1 } else { 0 };
        }
        let mut r = 1 as i64;
        for i in 2..=p {
            r = (r * i as i64) % MOD;
        }
        for i in 2..=(n - p) {
            r = (r * i as i64) % MOD;
        }
        r as i32
    }
}