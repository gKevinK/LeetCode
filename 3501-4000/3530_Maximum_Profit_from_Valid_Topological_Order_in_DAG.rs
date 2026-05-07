impl Solution {
    pub fn max_profit(n: i32, edges: Vec<Vec<i32>>, score: Vec<i32>) -> i32 {
        let un = n as usize;
        let mut g = vec![0; un];
        for e in &edges {
            g[e[1] as usize] |= 1 << e[0] as usize;
        }

        let mut cache = vec![-1; 1 << un];
        cache[0] = 0;
        let ALL = (1 << un) - 1;
        for m in 0..ALL {
            if cache[m] == -1 {
                continue;
            }
            let mult = m.count_ones() as i32 + 1;
            for i in 0..un {
                if (m & (1 << i) == 0) && (m & g[i] == g[i]) {
                    let m2 = m | (1 << i);
                    cache[m2] = cache[m2].max(cache[m] + score[i] * mult);
                }
            }
        }
        cache[ALL]
    }
}