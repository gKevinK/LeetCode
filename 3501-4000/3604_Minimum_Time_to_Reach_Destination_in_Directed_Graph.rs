impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let n = n as usize;
        let mut out = vec![vec![]; n];
        for e in &edges {
            out[e[0] as usize].push((e[1] as usize, e[2], e[3]));
        }
        let MAX = i32::MAX;
        let mut reach = vec![MAX; n];
        reach[0] = 0;
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0));
        while let Some((Reverse(t), i)) = heap.pop() {
            if reach[i] < t {
                continue;
            }
            if i == n - 1 {
                return reach[i];
            }
            for &(v, start, end) in &out[i] {
                let t2 = t.max(start) + 1;
                if t <= end && t2 < reach[v] {
                    reach[v] = t2;
                    heap.push((Reverse(t2), v));
                }
            }
        }
        -1
    }
}