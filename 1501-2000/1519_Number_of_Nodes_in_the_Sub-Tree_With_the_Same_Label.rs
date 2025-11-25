impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        let mut parent = vec![0; n];
        let mut order = Vec::with_capacity(n);
        order.push(0);
        for x in 0..n {
            let i = order[x];
            let p = parent[i];
            for &next in &g[i] {
                if next != p {
                    parent[next] = i;
                    order.push(next);
                }
            }
        }
        let label = labels.as_bytes();
        let mut res = vec![[0; 26]; n];
        for &i in order.iter().rev() {
            res[i][(label[i] - b'a') as usize] += 1;
            let p = parent[i];
            if i != 0 {
                for d in 0..26 {
                    res[p][d] += res[i][d];
                }
            }
        }
        (0..n).map(|i| res[i][(label[i] - b'a') as usize]).collect()
    }
}