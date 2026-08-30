impl Solution {
    pub fn generate_schedule(n: i32) -> Vec<Vec<i32>> {
        if n <= 4 {
            return vec![];
        }
        let mut res = vec![];
        for v in 0..n {
            res.push(vec![(v + 3) % n, (v + 2) % n]);
            res.push(vec![v, (v + 1) % n]);
        }
        for s in 2..=(n - 2) {
            for v in 1..=n {
                res.push(vec![v % n, (v + s) % n]);
            }
        }
        res
    }
}