impl Solution {
    fn pow(mut a: i64, mut b: i64, m: i64) -> i64 {
        let mut ans = 1i64;
        a %= m;
        while b > 0 {
            if b & 1 == 1 {
                ans = (ans % m) * (a % m) % m;
            }
            b /= 2;
            a = (a % m) * (a % m) % m;
        }
        ans
    }

    fn inv(x: i64, p: i64) -> i64 {
        Self::pow(x, p - 2, p)
    }

    fn comb(n: i64, m: i64, p: i64) -> i64 {
        if m > n {
            return 0;
        }
        let mut up = 1;
        let mut down = 1;
        for i in (n - m + 1)..=n {
            up = up * i % p;
        }
        for i in 1..=m {
            down = down * i % p;
        }
        up * Self::inv(down, p) % p
    }

    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        let m = 1_000_000_007;
        Self::comb((n + k - 1) as i64, std::cmp::min(n - 1, k) as i64, m) as i32
    }
}