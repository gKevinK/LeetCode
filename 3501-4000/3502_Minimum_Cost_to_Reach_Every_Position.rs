impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; cost.len()];
        result[0] = cost[0];
        for i in 1..cost.len() {
            result[i] = std::cmp::min(result[i - 1], cost[i]);
        }
        result
    }
}