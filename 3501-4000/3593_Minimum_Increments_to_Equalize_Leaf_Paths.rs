impl Solution {
    pub fn min_increase(n: i32, edges: Vec<Vec<i32>>, cost: Vec<i32>) -> i32 {
        let nu = n as usize;
        let mut g = vec![vec![]; nu];
        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        let mut sum = vec![0; nu];
        let mut res = 0;
        let mut queue = std::collections::VecDeque::from([(0, 0, 0), (0, 0, 1)]);
        while let Some((i, from, e)) = queue.pop_front() {
            if e == 0 {
                for &next in &g[i] {
                    if next != from {
                        queue.push_front((next, i, 1));
                        queue.push_front((next, i, 0));
                    }
                }
            } else {
                let mut max = 0;
                let mut max_n = 0;
                let mut count = 0;
                for &next in &g[i] {
                    if next != from {
                        count += 1;
                        if max < sum[next] {
                            max = sum[next];
                            max_n = 1;
                        } else if max == sum[next] {
                            max_n += 1;
                        }
                    }
                }
                res += count - max_n;
                sum[i] = max + cost[i] as i64;
            }
        }
        res
    }
}