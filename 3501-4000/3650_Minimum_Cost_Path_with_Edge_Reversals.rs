impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push((e[1] as usize, e[2]));
            g[e[1] as usize].push((e[0] as usize, e[2] * 2));
        }
        let mut visit = vec![i32::MAX; n];
        visit[0] = 0;
        let mut heap = std::collections::BinaryHeap::from([(0, 0)]);
        while let Some((_cost, i)) = heap.pop() {
            let cost = -_cost;
            if cost > visit[i] {
                continue;
            }
            if i == n - 1 {
                return cost;
            }
            for &(next, w) in &g[i] {
                if cost + w < visit[next] {
                    visit[next] = cost + w;
                    heap.push((-cost - w, next));
                }
            }
            visit[i] = 0;
        }
        -1
    }
}