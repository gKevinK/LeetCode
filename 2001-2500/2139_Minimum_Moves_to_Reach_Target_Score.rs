impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut t = target;
        let mut r = 0;
        let mut d = max_doubles;
        while t > 1 {
            if t % 2 == 0 {
                if d > 0 {
                    t /= 2;
                    d -= 1;
                } else {
                    return r + t - 1;
                }
            } else {
                t -= 1;
            }
            r += 1;
        }
        r
    }
}