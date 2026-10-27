impl Solution {
    pub fn minimum_flips(n: i32, edges: Vec<Vec<i32>>, start: String, target: String) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for (i, edge) in edges.iter().enumerate() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push((v, i as i32));
            adj[v].push((u, i as i32));
        }

        let s_bytes = start.as_bytes();
        let t_bytes = target.as_bytes();
        let mut diff = vec![0u8; n];
        let mut total_diff = 0;

        for i in 0..n {
            if s_bytes[i] != t_bytes[i] {
                diff[i] = 1;
                total_diff += 1;
            }
        }

        if total_diff % 2 != 0 {
            return vec![-1];
        }

        let mut order = Vec::with_capacity(n);
        let mut parent = vec![None; n];
        let mut edge_to_parent = vec![-1; n];
        let mut queue = std::collections::VecDeque::new();

        queue.push_back(0);
        parent[0] = Some(0);

        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &(v, idx) in &adj[u] {
                if parent[v].is_none() {
                    parent[v] = Some(u);
                    edge_to_parent[v] = idx;
                    queue.push_back(v);
                }
            }
        }

        let mut res = Vec::new();
        for &u in order.iter().rev() {
            if u == 0 {
                if diff[u] != 0 {
                    return vec![-1];
                }
                continue;
            }

            if diff[u] == 1 {
                let p = parent[u].unwrap();
                let edge_idx = edge_to_parent[u];
                res.push(edge_idx);
                diff[u] = 0;
                diff[p] ^= 1;
            }
        }
        res.sort_unstable();
        res
    }
}