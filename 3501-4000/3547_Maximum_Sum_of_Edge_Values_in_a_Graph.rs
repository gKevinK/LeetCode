impl Solution {
    pub fn max_score(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let nl = n as i64;
        let mut s = (nl - 1) * nl;
        for i in 1..(nl - 1) {
            s += i * (i + 2);
        }
        s + if edges.len() as i64 == nl { 2 } else { 0 }
    }
}