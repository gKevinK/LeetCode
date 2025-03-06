impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        
        let n = costs.len() / 2;
        let mut sum = 0;
        let mut h = BinaryHeap::new();
        for c in costs {
            sum += c[0];
            h.push(c[1] - c[0]);
            if h.len() > n {
                h.pop();
            }
        }
        for t in h {
            sum += t;
        }
        sum
    }
}