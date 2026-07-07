impl Solution {
    pub fn min_sensors(n: i32, m: i32, k: i32) -> i32 {
        ((n - 1) / (k * 2 + 1) + 1) * ((m - 1) / (k * 2 + 1) + 1)
    }
}