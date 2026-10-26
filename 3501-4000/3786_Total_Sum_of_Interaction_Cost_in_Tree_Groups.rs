impl Solution {
    pub fn interaction_costs(n: i32, edges: Vec<Vec<i32>>, group: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }

        let mut group_count = vec![0; 21];
        for &g in &group {
            group_count[g as usize] += 1;
        }

        let mut order = Vec::with_capacity(n);
        let mut parent = vec![n; n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);
        parent[0] = 0;
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &adj[u] {
                if parent[v] == n {
                    parent[v] = u;
                    queue.push_back(v);
                }
            }
        }

        let mut stree_count = vec![[0i64; 21]; n];
        let mut total_cost = 0i64;
        for &u in order.iter().rev() {
            let ug = group[u] as usize;
            stree_count[u][ug] += 1;
            if parent[u] != u {
                for g in 1..21 {
                    let s_child = stree_count[u][g];
                    let s_other = group_count[g] - s_child;
                    total_cost += s_child * s_other;
                    stree_count[parent[u]][g] += s_child;
                }
            }
        }
        total_cost
    }
}