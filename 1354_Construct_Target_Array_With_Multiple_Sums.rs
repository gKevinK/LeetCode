impl Solution {
    pub fn is_possible(mut target: Vec<i32>) -> bool {
        let mut sum = target.iter().map(|i| *i as i64).sum();
        let mut h = std::collections::BinaryHeap::new();
        for i in target {
            h.push(i as i64);
        }
        while let Some(mut v) = h.pop() {
            sum -= v;
            if v == 1 || sum == 1 {
                return true;
            }
            if v <= sum || sum == 0 || v % sum == 0 {
                return false;
            }
            v %= sum;
            sum += v;
            h.push(v);
        }
        false
    }
}