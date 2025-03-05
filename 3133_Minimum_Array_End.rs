impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut n1 = n - 1;
        let mut x1 = x;
        let mut res = 0i64;
        let mut b = 0;
        while n1 > 0 || x1 > 0 {
            if x1 & 1 == 1 {
                res |= 1 << b;
                x1 >>= 1;
            } else {
                res |= ((n1 & 1) as i64) << b;
                n1 >>= 1;
                x1 >>= 1;
            }
            b += 1
        }
        res
    }
}