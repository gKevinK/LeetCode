impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut r = 0i32;
        while n > 0 {
            r = n % 10 - r;
            n /= 10;
        }
        r
    }
}