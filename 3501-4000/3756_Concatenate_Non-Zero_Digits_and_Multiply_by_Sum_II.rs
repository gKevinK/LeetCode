const MOD: i64 = 1_000_000_007;
const LIMIT: usize = 100_001;

const POW10: [i64; LIMIT] = {
    let mut pow10 = [0i64; LIMIT];
    pow10[0] = 1;
    let mut i = 1;
    while i < LIMIT {
        pow10[i] = pow10[i - 1] * 10 % MOD;
        i += 1;
    }
    pow10
};

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut pref = vec![0i64; n + 1];
        let mut cnt = vec![0i32; n + 1];
        let mut dsum = vec![0i32; n + 1];

        for i in 0..n {
            let d = (bytes[i] - b'0') as i32;
            pref[i + 1] = if d != 0 { (pref[i] * 10 + d as i64) % MOD } else { pref[i] };
            cnt[i + 1]  = if d != 0 { cnt[i] + 1 } else { cnt[i] };
            dsum[i + 1] = dsum[i] + d;
        }

        let mut res = vec![0; queries.len()];
        for iq in 0..queries.len() {
            let l = queries[iq][0] as usize;
            let r = queries[iq][1] as usize + 1;
            let k = cnt[r] - cnt[l];
            if k == 0 {
                continue;
            }
            let x = (pref[r] - pref[l] * POW10[k as usize] % MOD + MOD) % MOD;
            let s = dsum[r] - dsum[l];
            res[iq] = ((s as i64 * x) % MOD) as i32;
        }
        res
    }
}