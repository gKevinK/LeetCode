impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut l = 0;
        let mut v = vec![ 0 ];
        for c in s.chars() {
            l ^= 1 << (c as i32 - 'a' as i32);
            v.push(l);
        }
        queries.iter().map(|q| {
            let mut diff = v[q[1] as usize + 1] ^ v[q[0] as usize];
            let mut n = 0;
            while diff > 0 {
                n += diff & 1;
                diff >>= 1;
            }
            n <= q[2] * 2 + if (q[1] - q[0] + 1) % 2 == 1 { 1 } else { 0 }
        }).collect()
    }
}