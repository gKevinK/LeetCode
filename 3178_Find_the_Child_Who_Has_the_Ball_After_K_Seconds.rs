impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        ((k + n - 1) % (2 * n - 2) - (n - 1)).abs()
    }
}