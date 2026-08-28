impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let n = order.len();
        let mut res = vec![];
        for i in 0..n {
            if friends.iter().any(|&x| order[i] == x) {
                res.push(order[i]);
            }
        }
        res
    }
}