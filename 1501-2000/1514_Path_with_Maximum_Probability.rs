impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
        let mut q = std::collections::BinaryHeap::new();
        let M = 1_000_000_000;
        q.push((M, start));
        let mut v = vec![0; n as usize];
        v[start as usize] = M;
        let mut m = vec![vec![]; n as usize];
        for i in 0..edges.len() {
            m[edges[i][0] as usize].push((edges[i][1], succ_prob[i]));
            m[edges[i][1] as usize].push((edges[i][0], succ_prob[i]));
        }
        while let Some((p, i)) = q.pop() {
            if i == end {
                return p as f64 / M as f64;
            }
            if p < v[i as usize] {
                continue;
            }
            for &l in &m[i as usize] {
                let p2 = (p as f64 * l.1) as i32;
                if p2 <= v[l.0 as usize] {
                    continue;
                }
                q.push((p2, l.0));
                v[l.0 as usize] = p2;
            }
        }
        0f64
    }
}