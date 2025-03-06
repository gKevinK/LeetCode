impl Solution {
    pub fn smallest_number(mut n: i32) -> i32 {
        let mut m = 1;
        while m <= n {
            n |= m;
            m <<= 1;
        }
        n
    }
}