impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let mut _n = n;
        let mut rev = 0;
        while _n > 0 {
            rev = rev * 10 + _n % 10;
            _n /= 10;
        }
        (n - rev).abs()
    }
}