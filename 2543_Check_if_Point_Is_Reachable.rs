impl Solution {
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        if x < y {
            let (mut x, mut y) = (y, x);
        }
        let mut z = x % y;
        while z > 0 && y > 1 {
            x = y;
            y = z;
            z = x % y;
        }
        y
    }

    pub fn is_reachable(target_x: i32, target_y: i32) -> bool {
        let mut n = Self::gcd(target_x, target_y);
        while n > 1 {
            if n % 2 == 1 {
                return false;
            }
            n /= 2;
        }
        true
    }
}