impl Solution {
    pub fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut r = 0;
        while n > 0 {
            r += n % k;
            n /= k;
        }
        r
    }
}