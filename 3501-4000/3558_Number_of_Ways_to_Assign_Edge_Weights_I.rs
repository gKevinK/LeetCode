impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n + 1];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut md = 0;
        let mut q = std::collections::VecDeque::with_capacity(n);
        q.push_back((1, 1, 0));
        while let Some((i, from, d)) = q.pop_front() {
            for &next in &g[i] {
                if next != from {
                    md = md.max(d + 1);
                    if g[next].len() > 1 {
                        q.push_back((next, i, d + 1));
                    }
                }
            }
        }
        let MOD = 1_000_000_007i64;
        if md > 0 { Self::pow_mod(2, md as i64 - 1, MOD) as i32 } else { 0 }
    }

    fn pow_mod(mut a: i64, mut p: i64, m: i64) -> i64 {
        let mut r = 1;
        while p > 0 {
            if p & 1 == 1 {
                r = (r * a) % m;
            }
            a = (a * a) % m;
            p >>= 1;
        }
        r
    }
}