impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut r = 0;
        if let Some(v) = left.into_iter().max() {
            r = std::cmp::max(r, v);
        }
        if let Some(v) = right.into_iter().min() {
            r = std::cmp::max(r, n - v);
        }
        r
    }
}