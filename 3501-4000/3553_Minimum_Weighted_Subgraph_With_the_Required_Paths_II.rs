impl Solution {
    pub fn minimum_weight(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];
        for e in edges {
            graph[e[0] as usize].push((e[1] as usize, e[2]));
            graph[e[1] as usize].push((e[0] as usize, e[2]));
        }
        let mut lift = vec![vec![0; n]];
        let mut psum = vec![0; n];
        let mut plen = vec![0; n];
        let mut q = std::collections::VecDeque::from([(0, 0)]);
        while let Some((i, from)) = q.pop_front() {
            for &(v, w) in &graph[i] {
                if v != from {
                    lift[0][v] = i;
                    psum[v] = psum[i] + w;
                    plen[v] = plen[i] + 1;
                    q.push_front((v, i));
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
            let a = q[0] as usize;
            let b = q[1] as usize;
            let c = q[2] as usize;
            let mut sum = 0;
            sum += psum[a] + psum[b] - psum[find_lca(a, b)] * 2;
            sum += psum[b] + psum[c] - psum[find_lca(b, c)] * 2;
            sum += psum[a] + psum[c] - psum[find_lca(a, c)] * 2;
            res.push(sum / 2);
        }
        res
    }
}