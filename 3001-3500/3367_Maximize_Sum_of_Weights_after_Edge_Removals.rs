impl Solution {
    pub fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push((e[1] as usize, e[2] as i64));
            g[e[1] as usize].push((e[0] as usize, e[2] as i64));
        }

        let mut sum = vec![(-1i64, -1i64); n];
        let mut stack = vec![(n, 0)];
        let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::with_capacity(k as usize + 1);
        // Start DFS here
        while let Some(&(s, t)) = stack.last() {
            if g[t].len() == 1 && g[t][0].0 == s {
                // If it's a leaf node
                sum[t] = (0, 0);
                stack.pop();
            } else if sum[t].0 == -1 {
                // If this is the first time to reach this node
                sum[t] = (0, -1_000_000_000_000);
                for e in &g[t] {
                    if e.0 != s {
                        stack.push((t, e.0));
                    }
                }
            } else {
                // If this is the second time to reach this node
                let (mut node_sum, mut node_sum_m) = (0i64, 0i64);
                if g[t].len() <= k as usize {
                    for &(next, w) in &g[t] {
                        let (s_del, s_use) = (sum[next].0, sum[next].1 + w);
                        node_sum += std::cmp::max(s_del, s_use);
                    }
                } else {
                    for &(next, w) in &g[t] {
                        let (s_del, s_use) = (sum[next].0, sum[next].1 + w);
                        let diff = s_use - s_del;
                        node_sum += s_del;
                        if diff > 0 {
                            // Keep max k diff in the heap
                            if heap.len() < k as usize || diff > heap.peek().unwrap().0 {
                                heap.push(Reverse(diff));
                                node_sum += diff;
                            }
                            if heap.len() > k as usize {
                                node_sum -= heap.pop().unwrap().0;
                            }
                        }
                    }
                    if heap.len() == k as usize {
                        node_sum_m = heap.peek().unwrap().0;
                    }
                    heap.clear();
                }
                sum[t] = (node_sum, node_sum - node_sum_m);
                stack.pop();
            }
        }
        sum[0].0
    }
}