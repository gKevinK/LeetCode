impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut dsum = 0;
        let mut dprod = 1;
        let mut n2 = n;
        while n2 > 0 {
            let d = n2 % 10;
            n2 /= 10;
            dsum += d;
            dprod *= d;
        }
        n % (dsum + dprod) == 0
    }
}