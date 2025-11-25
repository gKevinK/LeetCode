impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = coins.len();
        let mut g = vec![vec![]; n];
        let mut l = vec![0; n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
            l[u] += 1;
            l[v] += 1;
        }
        let mut queue = std::collections::VecDeque::from_iter((0..n).filter(|&i| coins[i] == 0 && l[i] == 1));
        let mut del = 0;
        while let Some(i) = queue.pop_front() {
            if l[i] == 0 {
                g[i].clear();
                continue;
            }
            l[i] -= 1;
            for p in g[i].drain(..) {
                if l[p] > 0 {
                    l[p] -= 1;
                    del += 1;
                    if coins[p] == 0 && l[p] == 1 {
                        queue.push_back(p);
                    }
                }
            }
        }
        queue.extend((0..n).filter(|&i| l[i] == 1));
        for _ in 0..2 {
            let mut size = queue.len();
            for _ in 0..size {
                let i = queue.pop_front().unwrap();
                if l[i] == 0 {
                    g[i].clear();
                    continue;
                }
                l[i] -= 1;
                for p in g[i].drain(..) {
                    if l[p] > 0 {
                        l[p] -= 1;
                        del += 1;
                        if l[p] == 1 {
                            queue.push_back(p);
                        }
                    }
                }
            }
        }
        (edges.len() as i32 - del) * 2
    }
}