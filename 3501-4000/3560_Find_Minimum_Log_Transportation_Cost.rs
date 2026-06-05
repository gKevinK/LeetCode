impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        if n > k {
            k as i64 * (n - k) as i64
        } else if m > k {
            k as i64 * (m - k) as i64
        } else {
            0
        }
    }
}