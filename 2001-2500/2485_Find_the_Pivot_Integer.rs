impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        for i in (n / 2)..=n {
            if i * (i + 1) == i + n * (n + 1) / 2 {
                return i;
            }
        }
        -1
    }
}