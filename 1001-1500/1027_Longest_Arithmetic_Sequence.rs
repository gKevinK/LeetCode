impl Solution {
    pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        
        let mut m: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
        let mut r = 0;
        for i in 0..a.len() {
            for j in (i + 1)..(a.len()) {
                let v = &mut m.entry(a[i] - a[j]).or_insert(Vec::new());
                v.push((i, j));
            }
        }
        for v in m.values() {
            let mut dp = vec![1; a.len()];
            for &(i, j) in v {
                dp[j] = dp[i] + 1;
                r = std::cmp::max(r, dp[j]);
            }
        }
        r
    }
}