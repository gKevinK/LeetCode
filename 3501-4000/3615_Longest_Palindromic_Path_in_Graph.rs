impl Solution {
    pub fn max_len(n: i32, edges: Vec<Vec<i32>>, label: String) -> i32 {
        let n = n as usize;
        let bytes = label.as_bytes();
        let mut count = [0; 26];
        for &b in bytes.iter() {
            count[(b - b'a') as usize] += 1;
        }
        let mut odd = 0;
        let mut max = 0;
        for i in 0..26 {
            if count[i] % 2 == 1 {
                odd = 1;
            }
            max += count[i] / 2 * 2;
        }
        max += odd;
        
        let mut queue = std::collections::VecDeque::from_iter((0..n).map(|i| (i, i, (1 << i) as usize)));
        let mut g = vec![vec![]; n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
            if bytes[u] == bytes[v] {
                queue.push_back((u.min(v), v.max(u), (1 << u) + (1 << v)));
            }
        }
        let mut visit = vec![false; n * n * (1 << n)];
        let mut res = 0;
        while let Some((u, v, mask)) = queue.pop_back() {
            res = res.max(mask.count_ones() as i32);
            if res == max {
                break;
            }
            for &un in &g[u] {
                if (1 << un) & mask > 0 {
                    continue;
                }
                for &vn in &g[v] {
                    if (1 << vn) & mask > 0 {
                        continue;
                    }
                    if un != vn && bytes[un] == bytes[vn] {
                        let mask2 = mask + (1 << un) + (1 << vn);
                        let tuple = (un.min(vn), un.max(vn), mask2);
                        let idx = (mask2 * n + tuple.0) * n + tuple.1;
                        if !visit[idx] {
                            visit[idx] = true;
                            queue.push_back(tuple);
                        }
                    }
                }
            }
        }
        res
    }
}