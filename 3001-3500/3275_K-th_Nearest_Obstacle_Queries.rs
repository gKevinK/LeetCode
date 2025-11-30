impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let ku = k as usize;
        let mut heap = std::collections::BinaryHeap::new();
        let mut res = Vec::with_capacity(queries.len());
        for q in &queries {
            let d = q[0].abs() + q[1].abs();
            let t = if heap.len() == ku { *heap.peek().unwrap() } else { 2_000_000_001 };
            if d <= t {
                heap.push(d);
            }
            if heap.len() > ku {
                heap.pop();
            }
            if heap.len() < ku {
                res.push(-1);
            } else {
                res.push(*heap.peek().unwrap());
            }
        }
        res
    }
}