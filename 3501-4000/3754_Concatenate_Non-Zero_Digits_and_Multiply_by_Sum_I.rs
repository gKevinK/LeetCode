impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut sum = 0;
        let mut con = 0;
        let mut m = 1;
        let mut n = n;
        while n > 0 {
            let d = n % 10;
            n /= 10;
            if d > 0 {
                sum += d;
                con += d * m;
                m *= 10;
            }
        }
        sum as i64 * con as i64
    }
}