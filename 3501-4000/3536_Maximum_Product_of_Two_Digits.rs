impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut x1 = 0;
        let mut x2 = 0;
        while n > 0 {
            let d = n % 10;
            n /= 10;
            if d > x1 {
                x2 = x1;
                x1 = d;
            } else if d > x2 {
                x2 = d;
            }
        }
        x1 * x2
    }
}