impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i64 {
        let bytes = s.as_bytes();
        let n = cost.len();
        let mut total = vec![0i64; 26];
        for i in 0..n {
            let idx = (bytes[i] - b'a') as usize;
            total[idx] += cost[i] as i64;
        }
        let mut sum = 0;
        let mut max = 0;
        for i in 0..26 {
            if total[i] > 0 {
                sum += total[i];
                max = max.max(total[i]);
            }
        }
        sum - max
    }
}