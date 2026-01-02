impl Solution {
    pub fn longest_special_path(edges: Vec<Vec<i32>>, nums: Vec<i32>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push((v, e[2]));
            g[v].push((u, e[2]));
        }
        let mut last = [0; 50001];
        let mut longest = 0;
        let mut min_count = i32::MAX;
        let mut path = Vec::with_capacity(n);
        let mut queue = std::collections::VecDeque::with_capacity(n);
        queue.push_back((0, 0, 0, 0, 1, 0, -1000000));
        while let Some((pop, i, from, mut sum, mut top, mut twice, w)) = queue.pop_front() {
            if pop == 0 {
                let prev = last[nums[i] as usize];
                path.push(w);
                last[nums[i] as usize] = path.len();
                while top <= prev.min(twice) {
                    sum -= path[top];
                    top += 1;
                }
                if prev > 0 {
                    twice = twice.max(prev);
                }
                if sum > longest {
                    longest = sum;
                    min_count = (path.len() - top + 1) as i32;
                } else if sum == longest {
                    min_count = min_count.min((path.len() - top + 1) as i32);
                }
                queue.push_front((1, i, 0, 0, prev, 0, 0));
                for &(next, w) in &g[i] {
                    if next != from {
                        queue.push_front((0, next, i, sum + w, top, twice, w));
                    }
                }
            } else {
                last[nums[i] as usize] = top; // prev
                path.pop();
            }
        }
        vec![longest, min_count]
    }
}