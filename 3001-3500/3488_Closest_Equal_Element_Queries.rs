impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let n = nums.len();
        let mut m = HashMap::new();
        for q in &queries {
            m.entry(nums[*q as usize]).or_insert((0, 0));
        }
        let mut dist = vec![-1; n];
        for i in 0..n {
            if let Some(t) = m.get_mut(&nums[i]) {
                if t.0 < n {
                    *t = (n + i, i);
                } else if t.1 + n == t.0 {
                    dist[t.1] = (i - t.1).min(t.0 - i) as i32;
                    dist[i] = dist[t.1];
                    t.1 = i;
                } else {
                    dist[i] = (t.0 - i).min(i - t.1) as i32;
                    dist[t.1] = dist[t.1].min((i - t.1) as i32);
                    dist[t.0 - n] = dist[t.0 - n].min((t.0 - i) as i32);
                    t.1 = i;
                }
            }
        }

        let mut res = Vec::with_capacity(queries.len());
        for q in queries {
            res.push(dist[q as usize]);
        }
        res
    }
}