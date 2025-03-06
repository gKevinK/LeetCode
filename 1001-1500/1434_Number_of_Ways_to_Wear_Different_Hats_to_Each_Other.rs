// impl Solution {
//     pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
//         let mut m = std::collections::HashMap::new();
//         m.insert(0u64, 1);
//         for p in hats {
//             let mut m2 = std::collections::HashMap::new();
//             for h in p {
//                 for (k, v) in &m {
//                     if *k & (1 << h) == 0 {
//                         let e = m2.entry(*k | (1 << h)).or_insert(0);
//                         *e = (*e + *v) % 1_000_000_007;
//                     }
//                 }
//             }
//             m = m2;
//         }
//         let mut r = 0;
//         for (_, v) in m {
//             r = (r + v) % 1_000_000_007;
//         }
//         r
//     }
// }

impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len();
        let MOD = 1_000_000_007;
        let mut h2p = vec![vec![]; 40];
        for (i, p) in hats.iter().enumerate() {
            for h in p {
                h2p[*h as usize - 1].push(i);
            }
        }
        let mut dp = vec![0; 1 << n];
        dp[0] = 1;
        for h in 0..40 {
            for m in (0..(1 << n)).rev() {
                for &p in &h2p[h] {
                    if (m & (1 << p)) == 0 {
                        dp[m | (1 << p)] += dp[m];
                        dp[m | (1 << p)] %= MOD;
                    }
                }
            }
        }
        *dp.last().unwrap()
    }
}