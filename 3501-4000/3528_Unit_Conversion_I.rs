impl Solution {
    pub fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = conversions.len();
        let MOD = 1_000_000_007i64;
        let mut res = vec![0; n + 1];
        res[0] = 1;

        let mut skip = 0;
        for c in &conversions {
            if res[c[0] as usize] != 0 {
                res[c[1] as usize] = ((res[c[0] as usize] as i64 * c[2] as i64) % MOD) as i32;
            } else {
                skip += 1;
            }
        }
        if skip > 0 {
            let mut g = vec![vec![]; n + 1];
            for c in conversions {
                g[c[0] as usize].push((c[1] as usize, c[2]));
            }
            let mut q = std::collections::VecDeque::from([0]);
            while let Some(i) = q.pop_front() {
                for &(next, f) in &g[i] {
                    res[next] = ((res[i] as i64 * f as i64) % MOD) as i32;
                    q.push_back(next);
                }
            }
        }
        res
    }
}