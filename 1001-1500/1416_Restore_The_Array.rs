impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let v: Vec<_> = s.bytes().collect();
        let mut dp = vec![0i64; v.len() + 1];
        dp[0] = 1;
        let MOD = 1_000_000_007i64;
        for i in 0..v.len() {
            let mut n = 0i64;
            let mut j = i;
            if v[j] == '0' as u8 {
                continue;
            }
            while j < v.len() {
                n = n * 10 + (v[j] - '0' as u8) as i64;
                if n > k as i64 {
                    break;
                }
                dp[j + 1] = (dp[i] + dp[j + 1]) % MOD;
                j += 1;
            }
        }
        *dp.last().unwrap() as i32
    }
}