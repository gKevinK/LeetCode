impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let MOD = 1_000_000_007;
        let mut r = 1i64;
        for i in 1..=n {
            r = r * (i * (2 * i - 1)) as i64 % MOD;
        }
        r as i32
    }
}