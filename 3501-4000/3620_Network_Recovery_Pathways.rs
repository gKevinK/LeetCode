impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let n = online.len();
        let mut g = vec![vec![]; n];
        let mut innum = vec![0; n];
        let mut costs = vec![];
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            if online[u] && online[v] {
                g[u].push((v, e[2]));
                innum[v] += 1;
                costs.push(e[2]);
            }
        }
        let mut order = Vec::with_capacity(n);
        order.extend((0..n).filter(|&i| online[i] && innum[i] == 0));
        for i in 0..n {
            if i >= order.len() {
                break;
            }
            for &(v, _) in &g[order[i]] {
                innum[v] -= 1;
                if innum[v] == 0 {
                    order.push(v);
                }
            }
        }
        costs.sort_unstable();
        costs.dedup();

        let mut lo = 0;
        let mut hi = costs.len();
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            let score = costs[mi];
            let mut dp = vec![i64::MAX; n];
            dp[0] = 0;
            for &i in &order {
                if dp[i] == i64::MAX {
                    continue;
                }
                for &(next, c) in &g[i] {
                    if c >= score {
                        let cost2 = dp[i] + c as i64;
                        if cost2 < dp[next] {
                            dp[next] = cost2;
                        }
                    }
                }
            }
            if dp[n - 1] <= k {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        if lo == 0 { -1 } else { costs[lo - 1] }
    }
}