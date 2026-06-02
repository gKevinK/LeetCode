impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];
        for e in edges {
            graph[e[0] as usize - 1].push(e[1] as usize - 1);
            graph[e[1] as usize - 1].push(e[0] as usize - 1);
        }
        let mut lift = vec![vec![0; n]];
        let mut plen = vec![0; n];
        let mut q = std::collections::VecDeque::from([(0, 0)]);
        while let Some((i, from)) = q.pop_front() {
            for &v in &graph[i] {
                if v != from {
                    lift[0][v] = i;
                    plen[v] = plen[i] + 1;
                    q.push_back((v, i));
                }
            }
        }
        let md: usize = *plen.iter().max().unwrap();
        for k in 1..=(md.ilog2() as usize) {
            lift.push(vec![0; n]);
            for i in 0..n {
                lift[k][i] = lift[k - 1][lift[k - 1][i]];
            }
        }

        let MOD = 1_000_000_007i64;
        let find_lca = |mut node1: usize, mut node2: usize| -> usize {
            if plen[node1] > plen[node2] {
                (node1, node2) = (node2, node1);
            }
            let (mut n1, mut n2) = (node1, node2);
            let d = plen[n2] - plen[n1];
            let mut exp = 0;
            while (1 << exp) <= d {
                if d & (1 << exp) > 0 {
                    n2 = lift[exp][n2];
                }
                exp += 1;
            }
            if n1 != n2 {
                for l in (0..lift.len()).rev() {
                    if l < lift.len() && lift[l][n1] != lift[l][n2] {
                        n1 = lift[l][n1];
                        n2 = lift[l][n2];
                    }
                }
                n1 = lift[0][n1];
                n2 = lift[0][n2];
            }
            n1
        };
        let mut res = Vec::with_capacity(queries.len());
        for q in queries {
            let n1 = q[0] as usize - 1;
            let n2 = q[1] as usize - 1;
            let lca = find_lca(n1, n2);
            let len = plen[n1] + plen[n2] - 2 * plen[lca];
            let r = if len > 0 { Self::pow_mod(2, len as i64 - 1, MOD) as i32 } else { 0 };
            res.push(r);
        }
        res
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