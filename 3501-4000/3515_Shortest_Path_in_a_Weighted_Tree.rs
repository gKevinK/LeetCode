impl Solution {
    pub fn tree_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let un = n as usize;
        let mut g = vec![vec![]; un + 1];
        for e in &edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut parent = vec![(0, 0); un + 1];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 1));
        while let Some((s, t)) = queue.pop_front() {
            parent[t] = (s, 0);
            for &next in &g[t] {
                if next != s {
                    queue.push_back((t, next));
                }
            }
        }
        for e in &edges {
            if parent[e[0] as usize].0 == e[1] as usize {
                parent[e[0] as usize].1 = e[2];
            } else {
                parent[e[1] as usize].1 = e[2];
            }
        }
        let mut skip: Vec<(usize, i32)> = vec![(0usize, 0i32); un + 1];
        let mut update: Vec<Vec<usize>> = vec![vec![]; un + 1];

        let mut res = vec![];
        for q in queries {
            if q[0] == 1 {
                let son = if parent[q[1] as usize].0 == q[2] as usize { q[1] } else { q[2] } as usize;
                for &u in &update[son] {
                    skip[u].1 = skip[u].1 + q[3] - parent[son].1;
                }
                parent[son].1 = q[3];
            } else {
                let mut curr = q[1] as usize;
                let mut sum = 0;
                let mut count = 0;
                while curr != 1 {
                    if skip[curr].0 != 0 {
                        sum += skip[curr].1;
                        curr = skip[curr].0;
                    } else {
                        sum += parent[curr].1;
                        curr = parent[curr].0;
                        count += 1;
                    }
                }
                res.push(sum);
                if count > 100 {
                    curr = q[1] as usize;
                    for _ in 0..(count % 100) {
                        curr = parent[curr].0;
                    }
                    while curr != 1 && skip[curr].0 == 0 {
                        let this = curr;
                        let mut s = 0;
                        for _ in 0..100 {
                            update[curr].push(this);
                            s += parent[curr].1;
                            curr = parent[curr].0;
                        }
                        skip[this] = (curr, s);
                    }
                }
            }
        }
        res
    }
}