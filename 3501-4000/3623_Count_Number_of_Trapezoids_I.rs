impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut m = std::collections::HashMap::new();
        for p in &points {
            *m.entry(p[1]).or_insert(0) += 1i64;
        }
        const MOD: i64 = 1_000_000_007;
        let mut res = 0;
        let mut c = 0;
        for &x in m.values() {
            if x > 1 {
                let comb = x * (x - 1) / 2 % MOD;
                res = (res + comb * c) % MOD;
                c = (c + comb) % MOD;
            }
        }
        res as _
    }
}